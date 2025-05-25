const PI: f32 = std::f32::consts::PI;

pub fn to_radians(deg: u32) -> f32 {
    deg as f32 * PI / 180.
}

pub fn max(rhs: f32, lhs: f32) -> f32 {
    if rhs > lhs {
        rhs
    } else {
        lhs
    }
}
