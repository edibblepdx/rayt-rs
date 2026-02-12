//! # Example
//!
//! Or define your scene in TOML using the [`scene_builder`].
//!
//! ```rust
//! use rayt_rs::prelude::*;
//!
//! const ASPECT_RATIO: f64 = 16.0 / 9.0;
//!
//! fn main() {
//!     // Safety: logging
//!     unsafe { std::env::set_var("RUST_LOG", "info") }
//!     env_logger::init();
//!
//!     let mut world = World::default();
//!
//!     let material_ground = world.add_material(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
//!     let material_center = world.add_material(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
//!     let material_left = world.add_material(Metal::new(Color::new(0.8, 0.8, 0.8)));
//!     let material_right = world.add_material(Metal::new(Color::new(0.8, 0.6, 0.2)));
//!
//!     [
//!         Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground),
//!         Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center),
//!         Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left),
//!         Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right),
//!     ]
//!     .iter()
//!     .for_each(|&o| world.add_object(o));
//!
//!     let camera = Camera::builder()
//!         .aspect_ratio(ASPECT_RATIO)
//!         .image_width(800usize)
//!         .sampler(SamplerConfig::Stratified { nx: 20, ny: 20 })
//!         .max_depth(50)
//!         .build();
//!
//!     camera.render(world);
//! }
//! ```
//! <div style="text-align: center;">
//!
//! ![result](https://github.com/user-attachments/assets/8699ffd4-24de-4041-b6d6-e54f62200b10)
//!
//! </div>

pub mod camera;
pub mod color;
pub mod hittable;
pub mod materials;
pub mod math;
pub mod ray;
pub mod samplers;
pub mod scene_builder;
pub mod world;

/// Commonly used items.
pub mod prelude {
    pub use crate::camera::Camera;
    pub use crate::color::Color;
    pub use crate::materials::Diffuse;
    pub use crate::materials::Lambertian;
    pub use crate::materials::Metal;
    pub use crate::materials::Normals;
    pub use crate::math::primitives::Sphere;
    pub use crate::math::types::Point3;
    pub use crate::math::types::UnitVec3;
    pub use crate::math::types::Vec3;
    pub use crate::samplers::SamplerConfig;
    pub use crate::world::World;
}
