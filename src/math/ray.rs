use super::{Point3, Vector3};

/// Struct to represent a ray that goes through `origin` in direction `direction`
/// The ray goes only in the positive direction and can be bounded
pub struct Ray {
    origin: Point3,
    direction: Vector3,
    max_t: f32,
}

impl Ray {
    /// Create a new ray
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction,
            max_t: f32::INFINITY,
        }
    }

    /// Adds a maximum bound to the ray
    pub fn set_bounds(self, max_t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            max_t,
        }
    }

    /// calculate the point on the ray for `t`
    pub fn at(&self, t: f32) -> Option<Point3> {
        if (0.0..self.max_t).contains(&t) {
            Some(self.origin + t * self.direction)
        } else {
            None
        }
    }

    pub fn dir(&self) -> &Vector3 {
        &self.direction
    }

    pub fn orig(&self) -> &Vector3 {
        &self.origin
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_ray() {
        let ray = Ray::new(Point3::new(1., 0., 0.), Vector3::new(1., 1., 1.));
        let p = ray.at(1.5);

        let expected = Point3::new(2.5, 1.5, 1.5);

        assert_eq!(p.unwrap(), expected);
    }
}
