use crate::math::{Point3, Ray, Vector3};

pub struct Intersection {
    pub point: Point3,
    pub t: f32,
    pub normal: Vector3,
}

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn intersection(&self, with: &Ray) -> Option<Intersection> {
        let oc = self.center - *with.orig();
        let a = with.dir().length_squared();
        let h = with.dir().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discr = h * h - a * c;
        if discr < 0. {
            return None;
        }

        let t = (h - discr.sqrt()) / (a);
        let p = with.at(t);
        let mut n = self.center - p;
        n.normalize();

        Some(Intersection {
            point: p,
            t,
            normal: n,
        })
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
        assert!(sphere.intersection(&two_hit).is_some());

        let no_hit = Ray::new(Point3::zero(), Vector3::new(0., 1., 1.));
        assert!(sphere.intersection(&no_hit).is_none());
    }
}
