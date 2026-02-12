use crate::{color::Color, hittable::HitRecord, ray::Ray};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, Ordering},
};

mod diffuse;
mod lambertian;
mod metal;
mod normals;

pub use diffuse::Diffuse;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use normals::Normals;

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize)]
pub struct MaterialId(pub u32);

type DynMaterial = Box<dyn Material + Send + Sync>;

#[derive(Default)]
pub struct MaterialMap(HashMap<MaterialId, DynMaterial>);

impl MaterialMap {
    pub fn insert<M>(&mut self, material: M) -> MaterialId
    where
        M: Material + Send + Sync + 'static,
    {
        let id = MaterialId(COUNTER.fetch_add(1, Ordering::Relaxed));
        self.0.insert(id, Box::new(material));
        id
    }

    pub fn get(&self, id: MaterialId) -> Option<&DynMaterial> {
        self.0.get(&id)
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;

    fn emitted(&self, _record: &HitRecord) -> Color {
        Color::BLACK
    }
}

#[derive(serde::Deserialize)]
pub struct TomlMaterial<M: Material> {
    pub id: MaterialId,
    #[serde(flatten)]
    pub data: M,
}
