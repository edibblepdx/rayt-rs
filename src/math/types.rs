//! This module defines types.

use crate::math::constants::INFINITY;

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

/// An interval defines a range over it's parameters.
#[derive(Clone, Copy, Debug)]
pub struct Interval(pub f64, pub f64);

impl Interval {
    /// Equivalent to (+[`INFINITY`], -[`INFINITY`])
    pub const EMPTY: Interval = Interval(INFINITY, -INFINITY);
    /// Equivalent to (-[`INFINITY`], +[`INFINITY`])
    pub const UNIVERSE: Interval = Interval(-INFINITY, INFINITY);

    /// Returns a new interval `(min, max)`
    pub fn new(min: f64, max: f64) -> Self {
        Interval(min, max)
    }

    /// Returns the size of the interval as `Interval.1 - Interval.0`.
    pub fn size(&self) -> f64 {
        self.1 - self.0
    }

    /// Returns `true` if the interval contains `x` bounds inclusive.
    pub fn contains(&self, x: f64) -> bool {
        (self.0..self.1).contains(&x)
    }

    /// Returns `true` if the interval contains `x` bounds non-inclusive.
    pub fn surrounds(&self, x: f64) -> bool {
        self.0 < x && x < self.1
    }
}

impl Default for Interval {
    /// Returns an [`Interval::EMPTY`] interval.
    fn default() -> Interval {
        Interval::EMPTY
    }
}

impl From<(f64, f64)> for Interval {
    fn from((min, max): (f64, f64)) -> Interval {
        Interval(min, max)
    }
}
