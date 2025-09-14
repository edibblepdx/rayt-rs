use crate::{camera::*, materials::*, math::primitives::*, world::World};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

pub struct SceneBuilder;

impl SceneBuilder {
    pub fn build(path: impl AsRef<Path>) -> Result<(Camera, World), Error> {
        let toml_str =
            fs::read_to_string(path).map_err(|err| Error::FileReadError(err.to_string()))?;

        let config: Config = toml::from_str(&toml_str)
            .map_err(|err| Error::ConfigDeError(err.message().to_string()))?;

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
pub enum Error {
    FileReadError(String),
    ConfigDeError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::FileReadError(s) => write!(f, "Config file read error: {s}"),
            Error::ConfigDeError(s) => write!(f, "Config deserialization error: {s}"),
        }
    }
}

impl std::error::Error for Error {}
