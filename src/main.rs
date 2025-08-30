use indicatif::ProgressIterator;
//use log::info;

use std::io;

use rayt_rs::color::{Color, write_color};
use rayt_rs::na::Vector3;

fn main() {
    env_logger::init();

    // Image

    let image_width: usize = 256;
    let image_height: usize = 256;

    let scale_x = 1.0 / (image_width - 1) as f64;
    let scale_y = 1.0 / (image_height - 1) as f64;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_color = Color(Vector3::new(j as f64 * scale_x, i as f64 * scale_y, 0.0));
            write_color(io::stdout(), &pixel_color).expect("Failed Write");
        }
    }
}
