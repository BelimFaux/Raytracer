use crate::math::{Color, Point3, Ray, Vector3};

#[derive(Clone)]
pub enum Light {
    Ambient { color: Color },
    Parallel { color: Color, direction: Vector3 },
    Point { color: Color, position: Point3 },
}

impl Light {
    /// Calculate the shadow ray to the object from the point `from`
    pub fn shadow_ray(&self, from: &Point3) -> Option<Ray> {
        const BIAS: f32 = 0.0001;
        match self {
            Self::Ambient { .. } => None,
            Self::Parallel {
                color: _,
                direction,
            } => {
                let direction = -*direction;
                let pos = *from + BIAS * direction;
                Some(Ray::new(pos, direction))
            }
            Self::Point { color: _, position } => {
                let direction = *position - *from;
                let length = direction.length();
                let pos = *from + BIAS * direction;
                Some(Ray::new(pos, direction).set_bounds(length))
            }
        }
    }
}
