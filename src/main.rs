use rayt_rs::math::primitives::Sphere;
use rayt_rs::prelude::*;
use rayt_rs::samplers::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    // Safety: just for logging
    unsafe { std::env::set_var("RUST_LOG", "info") }
    env_logger::init();

    let mut world = HittableList::default();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::builder()
        .aspect_ratio(ASPECT_RATIO)
        .image_width(400usize)
        .sampler(SamplerConfig::Stratified { nx: 5, ny: 5 })
        /*
        .sampler(SamplerConfig::Random {
            samples_per_pixel: 50,
        })
        */
        .max_depth(50)
        .build();

    camera.render(world);
}
