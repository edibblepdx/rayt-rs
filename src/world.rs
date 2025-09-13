//! The world stores all scene objects and materials.

use crate::{
    hittable::{Hittable, HittableList},
    materials::{Material, MaterialMap},
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
    pub fn add_material<T>(&mut self, k: i32, v: T)
    where
        T: Material + Send + Sync + 'static,
    {
        self.materials.insert(k, v);
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
