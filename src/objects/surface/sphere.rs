use crate::math::{Point3, Ray};

use super::{Intersection, Material};

/// struct to represent a Sphere in 3D-Space
#[derive(Clone, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

impl Sphere {
    /// Create a new sphere
    pub fn new(center: Point3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// Calculates the coefficients (a, h, c) of the intersection formula
    /// The ray direction should be normalized
    fn intersection_coefficients(&self, with: &Ray) -> (f32, f32) {
        let oc = *with.orig() - self.center;
        let b = oc.dot(with.dir());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let h = b * b - c;
        (b, h)
    }

    /// Test if the sphere intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        let (b, h) = self.intersection_coefficients(with);
        h >= 0. && with.t_in_range(-b - h.sqrt())
    }

    /// Calculates the intersection of the sphere and the `with` Ray if present
    /// Returns `None` if there is no intersection
    pub fn intersection(&self, with: &Ray) -> Option<Intersection> {
        let (b, h) = self.intersection_coefficients(with);
        if h < 0. {
            return None;
        }
        let h = h.sqrt();

        let t = if -b - h < 0. { -b + h } else { -b - h };
        let p = with.at(t);
        let mut n = p? - self.center;
        n.normalize();

        Some(Intersection {
            point: p?,
            t,
            normal: n,
            material: &self.material,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Color, Vector3};

    use super::*;

    #[test]
    fn sphere_intersection_test() {
        let sphere = Sphere::new(
            Point3::new(0., 0., -1.),
            0.5,
            Material::new(Color::new(0., 0., 0.), 0., 0., 0., (0., 0., 0., 1)),
        );

        let two_hit = Ray::new(Point3::zero(), Vector3::new(0., 0., -1.));
        assert!(sphere.intersection(&two_hit).is_some());

        let no_hit = Ray::new(Point3::zero(), Vector3::new(0., 1., 1.));
        assert!(sphere.intersection(&no_hit).is_none());

        let behind = Ray::new(Point3::zero(), Vector3::new(0., 0., 1.));
        assert!(sphere.intersection(&behind).is_none())
    }
}
