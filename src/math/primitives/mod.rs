//! Primitives module.

use crate::materials::MaterialId;

mod sphere;

pub use sphere::Sphere;

/// A marker trait for geometric primitives.
pub trait Primitive {}

/// Extension trait with required deserialization methods.
pub trait PrimitiveDeExtension {
    fn material_id(&self) -> MaterialId;
    fn with_material_id(self, id: MaterialId) -> Self;
}
