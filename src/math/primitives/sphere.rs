use crate::math::types::{Interval, Point3, UnitVec3};
use crate::{
    hittable::{HitRecord, Hittable},
    math::primitives::Primitive,
    ray::Ray,
};

#[derive(Copy, Clone, serde::Deserialize)]
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
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let (r_origin, r_direction) = (ray.origin(), ray.direction());

        let oc = self.center - r_origin;
        let a = r_direction.length_squared();
        let h = r_direction.dot(oc); // let b = -2h
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find smallest root in range.

        let sqrtd = discriminant.sqrt();

        let mut t = (h - sqrtd) / a;
        if !ray_t.surrounds(t) {
            t = (h + sqrtd) / a;
            if !ray_t.surrounds(t) {
                return None;
            }
        }

        let hit_point = ray.at(t);
        let outward_normal = UnitVec3::new_unchecked((hit_point - self.center) / self.radius);

        Some(HitRecord::new(ray, t, hit_point, outward_normal))
    }
}

mod tests {
    #[test]
    fn deserialize() {
        use crate::math::primitives::Sphere;
        use crate::math::types::*;
        use serde::Deserialize;

        let toml_str = r#"
            [primitive.sphere]
            center = [0.0, 0.0, -1.0]
            radius = 0.5
        "#;

        #[derive(Deserialize)]
        struct Config {
            primitive: Primitive,
        }

        #[derive(Deserialize)]
        struct Primitive {
            sphere: Sphere,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(Vec3::new(0.0, 0.0, -1.0) == config.primitive.sphere.center);
        assert!(0.5 == config.primitive.sphere.radius);
    }
}
