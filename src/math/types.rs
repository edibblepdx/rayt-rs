//! This module defines types.

use crate::math::constants::INFINITY;

use rand::Rng;

use std::ops::{Deref, Neg};

pub type Point3 = glam::f64::DVec3;
pub type Vec3 = glam::f64::DVec3;

#[derive(Copy, Clone)]
pub struct UnitVec3(glam::f64::DVec3);

impl UnitVec3 {
    /// Wraps the given [`Vec3`], assuming it is already normalized.
    pub fn new_normalize(v: Vec3) -> Self {
        UnitVec3(v.normalize())
    }

    /// Normalize the given [`Vec3`] and return it wrapped on a [`UnitVec3`].
    pub fn new_unchecked(v: Vec3) -> Self {
        UnitVec3(v)
    }

    /// Generates a random [`UnitVec3`] with uniform distribution.
    pub fn random(rng: &mut impl Rng) -> Self {
        use glam::f64::DVec3;
        loop {
            let v = DVec3::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            );
            let len = v.length();
            if 1e-160 < len && len <= 1.0 {
                return UnitVec3(v.normalize());
            }
        }
    }

    /// Generates a random [`UnitVec3`] in the same hemisphere as the normal.
    pub fn random_on_hemisphere(rng: &mut impl Rng, normal: UnitVec3) -> Self {
        let unit = UnitVec3::random(rng);
        if normal.dot(*unit) > 0.0 { unit } else { -unit }
    }

    /// Retrieves the underlying [`Vec3`].
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

    /// Clamps `x` to be within this interval.
    pub fn clamp(&self, x: f64) -> f64 {
        f64::max(f64::min(x, self.1), self.0)
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
