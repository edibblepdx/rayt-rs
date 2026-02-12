//! TOML scene builder.
//!
//! # Example
//!
//! ```toml
//! ## Create the camera.
//! [camera]
//! aspect_ratio = 1.777
//! image_width = 800
//! position = [0.0, 0.0, 0.0]
//! look_at = [0.0, 0.0, -1.0]
//! up = [0.0, 1.0, 0.0]
//! max_depth = 50
//!
//! ## Choose a sampler.
//! [camera.sampler]
//! type = "stratified"
//! nx = 20
//! ny = 20
//!
//! ## Provide an array of primitives that each reference a material.
//! [[primitive.sphere]]
//! center = [0.0, -100.5, -1.0]
//! radius = 100.0
//! material_id = 1
//!
//! ## Provide an array of materials.
//! [[material.lambertian]]
//! id = 1
//! albedo = [0.8, 0.8, 0.0]
//! ```
//! ```rust
//! use rayt_rs::scene_builder::SceneBuilder;
//!
//! fn main() -> anyhow::Result<()> {
//!     // Safety: just for logging
//!     unsafe { std::env::set_var("RUST_LOG", "info") }
//!     env_logger::init();
//!
//!     let (camera, world) = SceneBuilder::build("scene1.toml")?;
//!
//!     camera.render(world);
//!
//!     Ok(())
//! }
//! ```

use crate::{camera::*, materials::*, math::primitives::*, world::World};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

pub struct SceneBuilder;

impl SceneBuilder {
    pub fn build(path: impl AsRef<Path>) -> Result<(Camera, World), SceneError> {
        let toml_str =
            fs::read_to_string(path).map_err(|err| SceneError::FileReadError(err.to_string()))?;

        let config: Config = toml::from_str(&toml_str)
            .map_err(|err| SceneError::ConfigDeError(err.message().to_string()))?;

        let camera = config.camera.build();
        let mut world = World::default();

        let mut real_ids: HashMap<MaterialId, MaterialId> = HashMap::default();

        macro_rules! material {
            ($x:expr) => {
                if let Some(ms) = $x {
                    for m in ms {
                        let real_id = world.add_material(m.data);
                        real_ids.insert(m.id, real_id);
                    }
                }
            };
        }

        macro_rules! primitive {
            ($x:expr) => {
                if let Some(ps) = $x {
                    for mut p in ps {
                        if let Some(&real_id) = real_ids.get(&p.material_id) {
                            p.material_id = real_id;
                            world.add_object(p);
                        } else {
                            panic!("no material: {:?}", p.material_id);
                        }
                    }
                }
            };
        }

        material!(config.material.diffuse);
        material!(config.material.lambertian);
        material!(config.material.metal);
        material!(config.material.normals);

        primitive!(config.primitive.sphere);

        Ok((camera, world))
    }
}

#[derive(Deserialize)]
struct Config {
    camera: CameraBuilder,
    primitive: PrimitiveConfig,
    material: MaterialConfig,
}

#[derive(Deserialize)]
struct PrimitiveConfig {
    sphere: Option<Vec<Sphere>>,
}

#[derive(Deserialize)]
struct MaterialConfig {
    diffuse: Option<Vec<TomlMaterial<Diffuse>>>,
    lambertian: Option<Vec<TomlMaterial<Lambertian>>>,
    metal: Option<Vec<TomlMaterial<Metal>>>,
    normals: Option<Vec<TomlMaterial<Normals>>>,
}

#[derive(Debug)]
pub enum SceneError {
    FileReadError(String),
    ConfigDeError(String),
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SceneError::FileReadError(s) => write!(f, "Config file read error: {s}"),
            SceneError::ConfigDeError(s) => write!(f, "Config deserialization error: {s}"),
        }
    }
}

impl std::error::Error for SceneError {}
