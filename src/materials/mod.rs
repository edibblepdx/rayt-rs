use crate::{color::Color, hittable::HitRecord, ray::Ray};
use std::collections::HashMap;

mod diffuse;
mod lambertian;
mod metal;
mod normals;

pub use diffuse::Diffuse;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use normals::Normals;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, serde::Deserialize)]
pub struct MaterialId(pub u32);

type DynMaterial = Box<dyn Material + Send + Sync>;

#[derive(Default)]
pub struct MaterialMap {
    next_id: u32,
    map: HashMap<MaterialId, DynMaterial>,
}

impl MaterialMap {
    pub fn insert<M>(&mut self, material: M) -> MaterialId
    where
        M: Material + Send + Sync + 'static,
    {
        let id = MaterialId(self.next_id);
        self.map.insert(id, Box::new(material));
        self.next_id += 1;
        id
    }

    pub fn get(&self, id: MaterialId) -> Option<&DynMaterial> {
        self.map.get(&id)
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
