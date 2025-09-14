use crate::{color::Color, hittable::HitRecord, ray::Ray};

use std::collections::HashMap;

#[derive(Copy, Clone, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct MaterialId(u32);

impl From<u32> for MaterialId {
    fn from(id: u32) -> MaterialId {
        MaterialId(id)
    }
}

#[derive(Default)]
pub struct MaterialMap(HashMap<MaterialId, Box<dyn Material + Send + Sync>>);

impl MaterialMap {
    pub fn insert<K, V>(&mut self, k: K, v: V)
    where
        K: Into<MaterialId>,
        V: Material + Send + Sync + 'static,
    {
        self.0.insert(k.into(), Box::new(v));
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, attenuation: Color, scattered: &Ray) -> Option<HitRecord>;
}
