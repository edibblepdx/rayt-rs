//! This module defines a trait for hittable objects.

use crate::math::types::{Point3, UnitVec3};
use crate::ray::Ray;

pub enum FrontFace {
    Inside,
    Outside,
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
        let (front_face, normal) = if ray.direction().dot(&outward_normal) < 0.0 {
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
