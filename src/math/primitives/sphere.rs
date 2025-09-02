use crate::math::types::{Point3, UnitVec3};
use crate::{
    math::{
        hittable::{HitRecord, Hittable},
        primitives::Primitive,
    },
    ray::Ray,
};

#[derive(Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: impl Into<Point3>, radius: f64) -> Self {
        Sphere {
            center: center.into(),
            radius,
        }
    }
}

impl Primitive for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (r_origin, r_direction) = (ray.origin(), ray.direction());

        let oc = self.center - r_origin;
        let a = r_direction.length_squared();
        let h = r_direction.dot(oc); // let b = -2h
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let t = [(h - sqrtd) / a, (h + sqrtd) / a]
            .into_iter()
            .find(|&root| (t_min..=t_max).contains(&root))?;

        let hit_point = ray.at(t);
        let outward_normal = UnitVec3::new_unchecked((hit_point - self.center) / self.radius);

        eprintln!("{}", outward_normal.length());

        Some(HitRecord::new(ray, t, hit_point, outward_normal))
    }
}
