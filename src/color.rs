use crate::math::types::{Interval, Vec3};
use std::{
    fmt,
    ops::{Deref, DerefMut, Mul},
};

#[derive(Copy, Clone, PartialEq, serde::Deserialize)]
pub struct Color(pub Vec3);

impl Color {
    pub const BLACK: Color = Color(Vec3::splat(0.0));
    pub const WHITE: Color = Color(Vec3::splat(1.0));

    const GAMMA: f64 = 1.0 / 2.2;
    const INTENSITY: Interval = Interval(0.0, 0.999);

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
    }

    /// Converts linear to gamma.
    fn linear_to_gamma(self) -> Self {
        Color(self.powf(Color::GAMMA))
    }
}

impl Deref for Color {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Color {
        Color(value)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = self.linear_to_gamma();

        let r = (256.0 * Color::INTENSITY.clamp(c.0.x)) as u8;
        let g = (256.0 * Color::INTENSITY.clamp(c.0.y)) as u8;
        let b = (256.0 * Color::INTENSITY.clamp(c.0.z)) as u8;

        write!(f, "{r} {g} {b}")
    }
}

mod test {
    #[test]
    fn deserialize() {
        use super::*;
        use serde::Deserialize;

        let toml_str = r#"
            [material]
            albedo = [0.5, 0.5, 1.0]
        "#;

        #[derive(Deserialize)]
        struct Config {
            material: Mat,
        }

        #[derive(Deserialize)]
        struct Mat {
            albedo: Color,
        }

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(Color::new(0.5, 0.5, 1.0) == config.material.albedo);
    }
}
