use std::f32::consts::PI;

use crate::math::{Point3, Ray, Vec3};

use super::Texel;

/// struct to represent a Sphere in 3D-Space
#[derive(Clone, Debug)]
pub(super) struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    /// Create a new sphere
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    /// Calculates the coefficients (a, h, c) of the intersection formula
    fn intersection_coefficients(&self, with: &Ray) -> (f32, f32, f32) {
        let oc = self.center - *with.orig();
        let a = with.dir().length_squared();
        let h = with.dir().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        (a, h, c)
    }

    /// Test if any object intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        let (a, h, c) = self.intersection_coefficients(with);
        let discr = h * h - a * c;
        discr >= 0. && with.at((h - discr.sqrt()) / a).is_some()
    }

    /// Calculates the intersection of the sphere and the `with` Ray if present
    /// The normal in the intersection object will not necessarily be normalized
    /// Returns `None` if there is no intersection
    pub fn intersection(&self, with: &Ray) -> Option<(f32, Vec3, Texel)> {
        let (a, h, c) = self.intersection_coefficients(with);
        let discr = h * h - a * c;
        if discr < 0. {
            return None;
        }

        let discr = discr.sqrt();
        let t = if h - discr < 0. {
            (h + discr) / a
        } else {
            (h - discr) / a
        };
        let p = with.at(t)?;
        let n = p - self.center;

        Some((t, n, self.get_texel_at(&p)))
    }

    /// Compute the texel on the given point on the spheres surface
    /// Maps the texel according to [this](https://en.wikipedia.org/wiki/UV_mapping#Finding_UV_on_a_sphere) routine
    fn get_texel_at(&self, p: &Point3) -> Texel {
        let d = Vec3::normal(&(self.center - *p));
        let u = 0.5 + (d[0].atan2(d[2])) / (2. * PI);
        let v = 0.5 - (d[1].asin()) / (PI);

        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3;

    use super::*;

    #[test]
    fn sphere_intersection_test() {
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);

        let two_hit = Ray::new(Point3::zero(), Vec3::new(0., 0., -1.));
        assert!(sphere.intersection(&two_hit).is_some());

        let no_hit = Ray::new(Point3::zero(), Vec3::new(0., 1., 1.));
        assert!(sphere.intersection(&no_hit).is_none());

        let behind = Ray::new(Point3::zero(), Vec3::new(0., 0., 1.));
        assert!(sphere.intersection(&behind).is_none())
    }
}
