use crate::{color::Color, hittable::HitRecord, ray::Ray};

use std::collections::HashMap;

#[derive(Copy, Clone, serde::Deserialize)]
pub struct MaterialId(u32);

impl From<u32> for MaterialId {
    fn from(id: u32) -> MaterialId {
        MaterialId(id)
    }
}

#[derive(Default)]
pub struct MaterialMap(HashMap<i32, Box<dyn Material + Send + Sync>>);

impl MaterialMap {
    pub fn insert<T>(&mut self, k: i32, v: T)
    where
        T: Material + Send + Sync + 'static,
    {
        self.0.insert(k, Box::new(v));
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, attenuation: Color, scattered: &Ray) -> Option<HitRecord>;
}
