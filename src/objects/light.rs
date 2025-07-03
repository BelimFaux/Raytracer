use crate::math::{Color, Point3, Ray, Vec3, BIAS};

/// Enum to represent different types of light
#[derive(Clone, Debug)]
pub enum Light {
    Ambient { color: Color },
    Parallel { color: Color, direction: Vec3 },
    Point { color: Color, position: Point3 },
}

impl Light {
    /// Calculate the shadow ray to the object from the point `from`
    pub fn shadow_ray(&self, from: &Point3) -> Option<Ray> {
        match self {
            Self::Ambient { .. } => None,
            Self::Parallel {
                color: _,
                direction,
            } => {
                let direction = -Vec3::normal(direction);
                let pos = *from + BIAS * direction;
                Some(Ray::new(pos, direction))
            }
            Self::Point { color: _, position } => {
                let mut direction = *position - *from;
                let length = direction.length();
                direction /= length; // normalize
                let pos = *from + BIAS * direction;
                Some(Ray::new(pos, direction).set_bounds(length)) // bounds should be the initial
                                                                  // length
            }
        }
    }
}
