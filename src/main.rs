use indicatif::ProgressIterator;
//use log::info;

use std::io;

use rayt_rs::na::{point, vector};
use rayt_rs::{
    camera::Camera,
    color::{Color, write_color},
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    env_logger::init();

    // Camera

    let image_width: u32 = 400;
    let mut image_height: u32 = (image_width as f64 / ASPECT_RATIO) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;

    let focal_length = 10.0;

    let camera = Camera::new(
        point![0.0, 0.0, 0.0],  // eye
        vector![0.0, 1.0, 0.0], //up
        vector![1.0, 0.0, 0.0], //right
        viewport_width,
        viewport_height,
        focal_length,
    );

    // Image

    let image_width: usize = 256;
    let image_height: usize = 256;

    let scale_x = 1.0 / (image_width - 1) as f64;
    let scale_y = 1.0 / (image_height - 1) as f64;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_color = Color(vector![j as f64 * scale_x, i as f64 * scale_y, 0.0]);
            write_color(io::stdout(), &pixel_color).expect("Failed Write");
        }
    }
}
