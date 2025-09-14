use crate::math::types::{DVec3Extension, UnitVec3};
use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};

#[derive(serde::Deserialize)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = *record.normal + *UnitVec3::random(&mut rand::rng());

        if scatter_direction.near_zero() {
            scatter_direction = record.normal.into_inner();
        }

        let scattered_ray = Ray::new(record.hit_point, scatter_direction);
        Some((self.albedo, scattered_ray))
    }
}

mod tests {
    #[test]
    fn deserialize() {
        use super::*;
        use crate::materials::{MaterialId, TomlMaterial};
        use serde::Deserialize;

        let toml_str = r#"
            [material.lambertian]
            id = 1
            albedo = [0.5, 0.5, 0.5]
        "#;

        #[derive(Deserialize)]
        struct Config {
            material: MaterialConfig,
        }

        #[derive(Deserialize)]
        struct MaterialConfig {
            lambertian: TomlMaterial<Lambertian>,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(MaterialId(1) == config.material.lambertian.id);
        assert!(Color::new(0.5, 0.5, 0.5) == config.material.lambertian.data.albedo);
    }
}
