const PI: f32 = std::f32::consts::PI;

/// bias to prevent surface and shadow acne
pub const BIAS: f32 = 1e-4;

/// Convert degress to to radians
#[inline(always)]
pub fn to_radians(deg: f32) -> f32 {
    deg * PI / 180.
}

/// Determine the maximum of two f32's
#[inline(always)]
pub fn max(rhs: f32, lhs: f32) -> f32 {
    if rhs > lhs {
        rhs
    } else {
        lhs
    }
}

/// Determine the minimum of two f32's
#[inline(always)]
pub fn min(rhs: f32, lhs: f32) -> f32 {
    if rhs < lhs {
        rhs
    } else {
        lhs
    }
}
