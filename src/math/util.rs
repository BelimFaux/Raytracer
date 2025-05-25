const PI: f32 = std::f32::consts::PI;

pub fn to_radians(deg: u32) -> f32 {
    deg as f32 * PI / 180.
}
