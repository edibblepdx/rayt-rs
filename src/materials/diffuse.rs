use crate::math::types::UnitVec3;
use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};

#[derive(serde::Deserialize)]
pub struct Diffuse {
    albedo: Color,
}

impl Diffuse {
    pub fn new(albedo: Color) -> Self {
        Diffuse { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = UnitVec3::random_on_hemisphere(&mut rand::rng(), record.normal);
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
            [material.diffuse]
            id = 1
            albedo = [0.5, 0.5, 0.5]
        "#;

        #[derive(Deserialize)]
        struct Config {
            material: MaterialConfig,
        }

        #[derive(Deserialize)]
        struct MaterialConfig {
            diffuse: TomlMaterial<Diffuse>,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(MaterialId(1) == config.material.diffuse.id);
        assert!(Color::new(0.5, 0.5, 0.5) == config.material.diffuse.data.albedo);
    }
}
