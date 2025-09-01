//! This module defines a trait for hittable objects.

use crate::math::types::{Point3, Vec3};
use crate::ray::Ray;

pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
}

/// Allows a type to be tested for ray intersections.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
