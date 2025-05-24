use crate::math::{Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn intersection(&self, with: &Ray) -> bool {
        let oc = self.center - *with.orig();
        let a = with.dir().dot(&oc);
        let b = with.dir().dot(with.dir());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discr = a * a - b * c;
        discr >= 0.
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3;

    use super::*;

    #[test]
    fn sphere_intersection_test() {
        let sphere = Sphere::new(Point3::new(0., 0., -1.), 0.5);

        let two_hit = Ray::new(Point3::zero(), Vector3::new(0., 0., -1.));
        assert!(sphere.intersection(&two_hit));

        let no_hit = Ray::new(Point3::zero(), Vector3::new(0., 1., 1.));
        assert!(!sphere.intersection(&no_hit));
    }
}
