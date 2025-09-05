pub mod camera;
pub mod color;
pub mod hittable;
pub mod math;
pub mod ray;
pub mod scene_builder;

// reexport Hittable trait.
pub use hittable::Hittable;

pub mod prelude {
    pub use crate::camera::Camera;
    pub use crate::hittable::Hittable;
    pub use crate::hittable::HittableList;
    pub use crate::math::constants::INFINITY;
    pub use crate::math::constants::PI;
    pub use crate::math::types::Interval;
    pub use crate::math::types::Point3;
    pub use crate::math::types::UnitVec3;
    pub use crate::math::types::Vec3;
    pub use crate::ray::Ray;
}
