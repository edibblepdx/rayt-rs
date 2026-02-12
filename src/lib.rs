pub mod camera;
pub mod color;
pub mod hittable;
pub mod materials;
pub mod math;
pub mod ray;
pub mod samplers;
pub mod scene_builder;
pub mod world;

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
