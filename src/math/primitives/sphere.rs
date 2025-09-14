use crate::math::types::{Interval, Point3, UnitVec3};
use crate::{
    hittable::{HitRecord, Hittable},
    materials::MaterialId,
    math::primitives::{Primitive, PrimitiveDeExtension},
    ray::Ray,
};

#[derive(Copy, Clone, serde::Deserialize)]
pub struct Sphere {
    /// The center of the sphere.
    center: Point3,
    /// The radius of the sphere.
    radius: f64,
    /// The material id of the surface.
    material_id: MaterialId,
}

impl Sphere {
    pub fn new<P, M>(center: P, radius: f64, material_id: M) -> Self
    where
        P: Into<Point3>,
        M: Into<MaterialId>,
    {
        Sphere {
            center: center.into(),
            radius,
            material_id: material_id.into(),
        }
    }
}

impl Primitive for Sphere {}

impl PrimitiveDeExtension for Sphere {
    fn material_id(&self) -> MaterialId {
        self.material_id
    }

    fn with_material_id(mut self, id: MaterialId) -> Sphere {
        self.material_id = id;
        self
    }
}

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

        Some(HitRecord::new(
            ray,
            t,
            hit_point,
            outward_normal,
            self.material_id,
        ))
    }
}

mod tests {
    #[test]
    fn deserialize() {
        use super::*;
        use crate::math::types::*;
        use serde::Deserialize;

        let toml_str = r#"
            [primitive.sphere]
            center = [0.0, 0.0, -1.0]
            radius = 0.5
            material_id = 1
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
        assert!(MaterialId(1) == config.primitive.sphere.material_id);
    }
}
