use super::Vector3;

const PI: f32 = std::f32::consts::PI;

/// bias to prevent surface and shadow acne
pub const BIAS: f32 = 1e-5;

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

/// Calculate the reflected direction from an incident direction `d` and a normal `n`
pub fn reflect(d: Vector3, n: Vector3) -> Vector3 {
    2. * (-d.dot(&n)) * n + d
}
