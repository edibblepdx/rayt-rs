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

/// Writes the pixel color to `out`.
pub fn write_color(mut out: impl Write, pixel: &Color) -> Result<()> {
    let r = pixel.0.x;
    let g = pixel.0.y;
    let b = pixel.0.z;

    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;
    let ib = (255.999 * b) as u8;

    writeln!(out, "{ir} {ig} {ib}")
}
