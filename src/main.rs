use indicatif::ProgressIterator;
//use log::info;

use std::io;

use rayt_rs::math::types::*;
use rayt_rs::{
    //camera::Camera,
    color::{Color, write_color},
    math::{
        hittable::{Hittable, HittableList},
        primitives::Sphere,
    },
    ray::Ray,
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    env_logger::init();

    // Image
    // -----

    // Image dimensions are integer-valued.
    let image_width: u32 = 400;
    let mut image_height: u32 = (image_width as f64 / ASPECT_RATIO) as u32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World
    // -----
    let mut world = HittableList::default();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    // ------

    // Distance from the eye to the image plane.
    let focal_length = 1.0;

    // Viewport dimensions are real-valued.
    // The actual aspect ratio may not be ASPECT_RATIO.
    let viewport_height = 2.0; // arbitrary
    let viewport_width = viewport_height * image_width as f64 / image_height as f64;

    let eye = Vec3::new(0.0, 0.0, 0.0);

    /*
    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),  // eye
        Vec3::new(0.0, 1.0, 0.0), // up
        Vec3::new(1.0, 0.0, 0.0), // right
        viewport_width,
        viewport_height,
        focal_length,
    );
    */

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        eye - Vec3::new(viewport_width / 2.0, -viewport_height / 2.0, focal_length);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    // ------

    println!("P3\n{image_width} {image_height}\n255");

    for j in (0..image_height).progress() {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = UnitVec3::new_normalize(pixel_center - eye);
            let r = Ray::new(eye, ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(io::stdout(), &pixel_color).expect("Failed Write");
        }
    }
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    if let Some(record) = world.hit(ray, 0.0, 10.0) {
        let mapped = record.normal.map(|e| (e + 1.0) / 2.0);
        return Color(mapped);
    }

    let mut t = ray.direction().y;
    t = (t + 1.0) / 2.0;

    let start = Color(Vec3::new(1.0, 1.0, 1.0));
    let end = Color(Vec3::new(0.5, 0.7, 1.0));

    Color((1.0 - t) * start.0 + t * end.0)
}
