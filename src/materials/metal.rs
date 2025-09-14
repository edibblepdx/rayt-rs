use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};

#[derive(serde::Deserialize)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let refelcted_direction = ray.direction().reflect(*record.normal);
        let scattered_ray = Ray::new(record.hit_point, refelcted_direction);
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
            [material.metal]
            id = 1
            albedo = [0.5, 0.5, 0.5]
        "#;

        #[derive(Deserialize)]
        struct Config {
            material: MaterialConfig,
        }

        #[derive(Deserialize)]
        struct MaterialConfig {
            metal: TomlMaterial<Metal>,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(MaterialId(1) == config.material.metal.id);
        assert!(Color::new(0.5, 0.5, 0.5) == config.material.metal.data.albedo);
    }
}
