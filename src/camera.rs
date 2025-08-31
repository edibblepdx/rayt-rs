use crate::types::{Point3, UnitVec3, Vec3};

pub struct Camera {
    pub eye: Point3,
    pub up: UnitVec3,
    pub right: UnitVec3,
    pub forward: UnitVec3,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
}

impl Camera {
    /// Returns a new camera with the given parameters.
    ///
    /// The forward direction is calculated as `right` cross `up`.
    pub fn new(
        eye: Point3,
        up: Vec3,
        right: Vec3,
        viewport_width: f64,
        viewport_height: f64,
        focal_length: f64,
    ) -> Self {
        let up = UnitVec3::new_normalize(up);
        let right = UnitVec3::new_normalize(right);
        let forward = UnitVec3::new_normalize(right.cross(&up));

        Camera {
            eye: eye.into(),
            up,
            right,
            forward,
            viewport_width,
            viewport_height,
            focal_length,
        }
    }
}
