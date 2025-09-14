use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray};

#[derive(serde::Deserialize)]
pub struct Normals;

impl Normals {
    pub fn new() -> Self {
        Normals {}
    }
}

impl Default for Normals {
    fn default() -> Self {
        Normals::new()
    }
}

impl Material for Normals {
    fn scatter(&self, _ray: &Ray, _record: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, record: &HitRecord) -> Color {
        Color(record.normal.map(|e| (e + 1.0) / 2.0))
    }
}

mod tests {
    #[test]
    fn deserialize() {
        use super::*;
        use crate::materials::{MaterialId, TomlMaterial};
        use serde::Deserialize;

        let toml_str = r#"
            [material.normals]
            id = 1
        "#;

        #[derive(Deserialize)]
        struct Config {
            material: MaterialConfig,
        }

        #[derive(Deserialize)]
        struct MaterialConfig {
            normals: TomlMaterial<Normals>,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(MaterialId(1) == config.material.normals.id);
    }
}
