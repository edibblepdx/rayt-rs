//! This module defines types using glam.

use std::ops::{Deref, Neg};

pub type Point3 = glam::f64::DVec3;
pub type Vec3 = glam::f64::DVec3;

#[derive(Copy, Clone)]
pub struct UnitVec3(glam::f64::DVec3);

impl UnitVec3 {
    pub fn new_normalize(v: Vec3) -> Self {
        UnitVec3(v.normalize())
    }

    pub fn new_unchecked(v: Vec3) -> Self {
        UnitVec3(v)
    }

    pub fn into_inner(self) -> Vec3 {
        self.0
    }
}

impl From<glam::f64::DVec3> for UnitVec3 {
    fn from(v: glam::f64::DVec3) -> UnitVec3 {
        UnitVec3::new_normalize(v)
    }
}

impl Deref for UnitVec3 {
    type Target = glam::f64::DVec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Neg for UnitVec3 {
    type Output = UnitVec3;

    fn neg(self) -> Self::Output {
        UnitVec3(-self.0)
    }
}
