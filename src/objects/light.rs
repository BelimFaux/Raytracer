use crate::math::{Color, Point3, Ray, Vec3, BIAS};

/// Enum to represent different types of light
#[derive(Clone, Debug)]
pub enum Light {
    Ambient {
        color: Color,
    },
    Parallel {
        color: Color,
        direction: Vec3,
    },
    Point {
        color: Color,
        position: Point3,
    },
    Spot {
        color: Color,
        position: Point3,
        direction: Vec3,
        falloff: (f32, f32),
    },
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
            Self::Spot {
                color: _,
                position,
                direction,
                falloff,
            } => {
                let mut shadow_direction = *position - *from;
                let length = shadow_direction.length();
                shadow_direction /= length;

                // if the point is completely outside the cone, we dont have to send a shadow ray
                let light_dir = -Vec3::normal(direction);
                let limit = falloff.1;
                if light_dir.dot(&shadow_direction) < limit {
                    None
                } else {
                    let pos = *from + BIAS * shadow_direction;
                    Some(Ray::new(pos, shadow_direction).set_bounds(length))
                }
            }
        }
    }
}
