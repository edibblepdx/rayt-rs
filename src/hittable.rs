//! This module defines a trait for hittable objects.

use crate::math::types::{Interval, Point3, UnitVec3};
use crate::ray::Ray;

#[derive(Default)]
pub enum FrontFace {
    #[default]
    Outside,
    Inside,
}

pub struct HitRecord {
    /// The ray parameter.
    pub t: f64,
    /// The intersection point of the ray.
    pub hit_point: Point3,
    /// The surface normal.
    pub normal: UnitVec3,
    /// The front face of the surface in relation to the ray.
    pub front_face: FrontFace,
}

impl HitRecord {
    /// Constructs a new `HitRecord`.
    pub fn new(ray: &Ray, t: f64, hit_point: Point3, outward_normal: UnitVec3) -> Self {
        let (front_face, normal) = if ray.direction().dot(*outward_normal) < 0.0 {
            (FrontFace::Outside, outward_normal)
        } else {
            (FrontFace::Inside, -outward_normal)
        };

        HitRecord {
            t,
            hit_point,
            normal,
            front_face,
        }
    }
}

/// Allows a type to be tested for ray intersections.
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList(Vec<Box<dyn Hittable + Send + Sync>>);

impl HittableList {
    pub fn add<T>(&mut self, o: T)
    where
        T: Hittable + Send + Sync + 'static,
    {
        self.0.push(Box::new(o))
    }
}

impl Hittable for HittableList {
    /// Iterate through all hittable objects to find the closest hit.
    fn hit(&self, ray: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        let mut record = None;
        for o in &self.0 {
            record = o.hit(ray, ray_t).map_or(record, |r| {
                ray_t.1 = r.t;
                Some(r)
            })
        }

        record
    }
}
