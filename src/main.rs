use rayt_rs::prelude::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    // Safety: logging
    unsafe { std::env::set_var("RUST_LOG", "info") }
    env_logger::init();

    let mut world = World::default();

    let material_ground = world.add_material(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = world.add_material(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = world.add_material(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = world.add_material(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add_object(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add_object(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add_object(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add_object(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let camera = Camera::builder()
        .aspect_ratio(ASPECT_RATIO)
        .image_width(800usize)
        .sampler(SamplerConfig::Stratified { nx: 20, ny: 20 })
        .max_depth(50)
        .build();

    camera.render(world);
}
