use crate::{camera::*, hittable::HittableList, math::primitives::*};

use std::{fs, path::Path};

use serde::Deserialize;

pub struct SceneBuilder;

impl SceneBuilder {
    pub fn build(path: impl AsRef<Path>) -> anyhow::Result<(Camera, HittableList)> {
        let toml_str = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&toml_str)?;

        let camera = config.camera.build();
        let mut world = HittableList::default();

        for sphere in config.primitive.sphere {
            world.add(sphere);
        }

        Ok((camera, world))
    }
}

#[derive(Deserialize)]
struct Config {
    camera: CameraBuilder,
    primitive: Primitive,
}

#[derive(Deserialize)]
struct Primitive {
    sphere: Vec<Sphere>,
}
