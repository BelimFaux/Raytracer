const PI: f32 = std::f32::consts::PI;

/// Convert degress to to radians
pub fn to_radians(deg: u32) -> f32 {
    deg as f32 * PI / 180.
}

/// Determine the maximum of two f32's
pub fn max(rhs: f32, lhs: f32) -> f32 {
    if rhs > lhs {
        rhs
    } else {
        lhs
    }
}
