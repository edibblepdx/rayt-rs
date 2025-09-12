use crate::math::types::Vec3;

use std::{
    fmt,
    ops::{Deref, DerefMut, Mul},
};

#[derive(Copy, Clone)]
pub struct Color(pub Vec3);

impl Color {
    pub const BLACK: Color = Color(Vec3::splat(0.0));
    pub const WHITE: Color = Color(Vec3::splat(1.0));
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
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs)
    }
}

/// Converts linear to gamma.
fn linear_to_gamma(c: Color) -> Color {
    Color(c.powf(1.0 / 2.2))
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = linear_to_gamma(*self);

        let ir = (255.999 * c.0.x) as u8;
        let ig = (255.999 * c.0.y) as u8;
        let ib = (255.999 * c.0.z) as u8;

        write!(f, "{ir} {ig} {ib}")
    }
}
