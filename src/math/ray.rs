use super::{Point3, Vec3};

/// Struct to represent a ray that goes through `origin` in direction `direction`
/// The ray goes only in the positive direction and can be bounded
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    max_t: f32,
}

impl Ray {
    /// Create a new ray
    #[inline]
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
            max_t: f32::INFINITY,
        }
    }

    /// Adds a maximum bound to the ray
    #[inline]
    pub fn set_bounds(self, max_t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            max_t,
        }
    }

    /// calculate the point on the ray for `t`
    #[inline]
    pub fn at(&self, t: f32) -> Option<Point3> {
        if (0.0..=self.max_t).contains(&t) {
            Some(self.origin + t * self.direction)
        } else {
            None
        }
    }

    /// determine if t value is in range for this ray
    #[inline]
    pub fn t_in_range(&self, t: f32) -> bool {
        (0.0..=self.max_t).contains(&t)
    }

    /// get the direction of the ray
    #[inline]
    pub fn dir(&self) -> &Vec3 {
        &self.direction
    }

    /// get the origin of the ray
    #[inline]
    pub fn orig(&self) -> &Vec3 {
        &self.origin
    }

    /// get the maximum t bound
    #[inline]
    pub fn max_t(&self) -> f32 {
        self.max_t
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at_ray() {
        let ray = Ray::new(Point3::new(1., 0., 0.), Vec3::new(1., 1., 1.));
        let p = ray.at(1.5);

        let expected = Point3::new(2.5, 1.5, 1.5);

        assert_eq!(p.unwrap(), expected);
    }

    #[test]
    fn point_notat_ray_with_bounds() {
        let ray = Ray::new(Point3::new(1., 0., 0.), Vec3::new(1., 1., 1.)).set_bounds(1.0);
        let p = ray.at(1.5);

        assert!(p.is_none());
    }
}
