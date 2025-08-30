use na::Vector3;
use std::io::{Result, Write};

#[derive(Copy, Clone)]
pub struct Color(pub Vector3<f64>);

pub fn write_color(mut out: impl Write, pixel: &Color) -> Result<()> {
    let r = pixel.0.x;
    let g = pixel.0.y;
    let b = pixel.0.z;

    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;
    let ib = (255.999 * b) as u8;

    writeln!(out, "{ir} {ig} {ib}")
}
