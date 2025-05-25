use crate::math::{Color, Point3, Ray, Vector3};

use super::Light;

pub struct Intersection<'a> {
    pub point: Point3,
    pub t: f32,
    pub normal: Vector3,
    pub material: &'a Material,
}

impl Intersection<'_> {
    pub fn get_color(&self, light: &Light) -> Color {
        self.material.get_color(&self.point, &self.normal, light)
    }
}

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
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
            material: &self.material,
        })
    }
}

#[derive(Clone)]
pub struct Material {
    color: Color,
    ka: f32,
    kd: f32,
    ks: f32,
    exp: i32,
}

impl Material {
    pub fn new(color: Color, ka: f32, kd: f32, ks: f32, exp: i32) -> Material {
        Material {
            color,
            ka,
            kd,
            ks,
            exp,
        }
    }

    pub fn get_color(&self, _point: &Point3, _normal: &Vector3, light: &Light) -> Color {
        match light {
            Light::Ambient { color: _ } => self.color * self.ka,
            Light::Parallel {
                color: _,
                direction: _,
            } => self.color,
            Light::Point {
                color: _,
                position: _,
            } => self.color,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vector3;

    use super::*;

    #[test]
    fn sphere_intersection_test() {
        let sphere = Sphere::new(
            Point3::new(0., 0., -1.),
            0.5,
            Material::new(Color::new(0., 0., 0.), 0., 0., 0., 1),
        );

        let two_hit = Ray::new(Point3::zero(), Vector3::new(0., 0., -1.));
        assert!(sphere.intersection(&two_hit).is_some());

        let no_hit = Ray::new(Point3::zero(), Vector3::new(0., 1., 1.));
        assert!(sphere.intersection(&no_hit).is_none());
    }
}
