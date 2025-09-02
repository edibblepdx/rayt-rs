use crate::math::types::{Point3, UnitVec3};

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point3,
    direction: UnitVec3,
}

impl Ray {
    /// Constructs a new ray with an origin and direction.
    pub fn new(origin: impl Into<Point3>, direction: impl Into<UnitVec3>) -> Self {
        Ray {
            origin: origin.into(),
            direction: direction.into(),
        }
    }

    /// Returns the origin of the ray.
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// Returns the direction of the ray.
    pub fn direction(&self) -> UnitVec3 {
        self.direction
    }

    /// Returns a point along the ray.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction.into_inner()
    }
}
