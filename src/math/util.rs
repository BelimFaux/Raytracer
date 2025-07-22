use std::ops::{Add, Mul};

const PI: f32 = std::f32::consts::PI;

/// bias to prevent surface and shadow acne
pub const BIAS: f32 = 1e-4;

/// Convert degress to to radians
#[inline]
#[must_use]
pub fn to_radians(deg: f32) -> f32 {
    deg * PI / 180.
}

/// Determine the maximum of two f32's
#[inline]
#[must_use]
pub fn max(rhs: f32, lhs: f32) -> f32 {
    if rhs > lhs {
        rhs
    } else {
        lhs
    }
}

/// Determine the minimum of two f32's
#[inline]
#[must_use]
pub fn min(rhs: f32, lhs: f32) -> f32 {
    if rhs < lhs {
        rhs
    } else {
        lhs
    }
}

/// Linearly interpolates between two values with percentage `w`
#[inline]
#[must_use]
pub fn lerp<T>(a: T, b: T, w: f32) -> T
where
    T: Add<Output = T> + Mul<f32, Output = T>,
{
    a * (1. - w) + b * w
}

/// clamp a value between two edges smoothly by using hermite interpolation
/// See [https://en.wikipedia.org/wiki/Smoothstep](https://en.wikipedia.org/wiki/Smoothstep)
#[inline]
#[must_use]
pub fn smoothstep(edge0: f32, edge1: f32, t: f32) -> f32 {
    let x = ((t - edge0) / (edge1 - edge0)).clamp(0., 1.);
    x * x * (3. - 2. * x)
}
