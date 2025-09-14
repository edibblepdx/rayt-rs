//! The world stores all scene objects and materials.

use crate::{
    hittable::{Hittable, HittableList},
    materials::{Material, MaterialId, MaterialMap},
};

#[derive(Default)]
pub struct World {
    /// The list of hittable objects in the world.
    objects: HittableList,
    /// The map of materials in the world.
    materials: MaterialMap,
}

impl World {
    /// Adds an object to the world.
    pub fn add_object<T>(&mut self, o: T)
    where
        T: Hittable + Send + Sync + 'static,
    {
        self.objects.add(o);
    }

    /// Adds a material to the world.
    pub fn add_material<M>(&mut self, material: M) -> MaterialId
    where
        M: Material + Send + Sync + 'static,
    {
        self.materials.insert(material)
    }

    /// Returns an immutable reference to hittable objects.
    pub fn objects(&self) -> &HittableList {
        &self.objects
    }

    /// Returns an immutable reference to materials.
    pub fn materials(&self) -> &MaterialMap {
        &self.materials
    }
}
