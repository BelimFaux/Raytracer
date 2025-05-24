use super::{Point3, Vector3};

/// Struct to represent a ray that goes through `origin` in direction `direction`
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    /// Create a new ray
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    /// calculate the point on the ray for `t`
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
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

        assert_eq!(p, expected);
    }
}
