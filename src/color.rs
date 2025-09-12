use std::{
    io::{Result, Write},
    ops::{Deref, DerefMut},
};

use crate::math::types::Vec3;

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

/// Converts linear to gamma.
pub fn linear_to_gamma(c: Color) -> Color {
    Color(c.powf(1.0 / 2.2))
}

/// Writes the pixel color to `out`.
pub fn write_color(mut out: impl Write, pixel: Color) -> Result<()> {
    let c = linear_to_gamma(pixel);

    let ir = (255.999 * c.0.x) as u8;
    let ig = (255.999 * c.0.y) as u8;
    let ib = (255.999 * c.0.z) as u8;

    writeln!(out, "{ir} {ig} {ib}")
}
