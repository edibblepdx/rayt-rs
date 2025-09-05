use crate::{camera::*, hittable::HittableList, math::primitives::*};

use std::{fs, path::Path};

use serde::Deserialize;

pub struct SceneBuilder;

impl SceneBuilder {
    pub fn build(path: impl AsRef<Path>) -> Result<(Camera, HittableList), Error> {
        let toml_str =
            fs::read_to_string(path).map_err(|err| Error::FileReadError(err.to_string()))?;

        let config: Config = toml::from_str(&toml_str)
            .map_err(|err| Error::ConfigDeError(err.message().to_string()))?;

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

#[derive(Debug)]
pub enum Error {
    FileReadError(String),
    ConfigDeError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::FileReadError(s) => write!(f, "File read error: {}", s),
            Error::ConfigDeError(s) => write!(f, "Config deserialization error: {}", s),
        }
    }
}

impl std::error::Error for Error {}
