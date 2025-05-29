use crate::math::{max, Color, Point3, Ray, Vector3};

use super::Light;

/// struct to represent a Sphere in 3D-Space
#[derive(Clone)]
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
    /// Returns `None` if there is no intersection
    pub fn intersection(&self, with: &Ray) -> Option<Intersection> {
        let (a, h, c) = self.intersection_coefficients(with);
        let discr = h * h - a * c;
        if discr < 0. {
            return None;
        }

        let t = (h - discr.sqrt()) / (a);
        let p = with.at(t);
        let mut n = self.center - p?;
        n.normalize();

        Some(Intersection {
            point: p?,
            t,
            normal: n,
            material: &self.material,
        })
    }
}

/// Struct to represent a Material
#[derive(Clone)]
pub struct Material {
    color: Color,
    ka: f32,
    kd: f32,
    ks: f32,
    exp: u32,
}

impl Material {
    /// Create a new material
    pub fn new(color: Color, ka: f32, kd: f32, ks: f32, exp: u32) -> Material {
        Material {
            color,
            ka,
            kd,
            ks,
            exp,
        }
    }

    /// Calculate the color of the material with a light color
    fn phong(
        &self,
        light_color: &Color,
        neg_light: &Vector3,
        vnormal: &Vector3,
        neg_veye: &Vector3,
    ) -> Color {
        let l = Vector3::normal(neg_light);
        let n = Vector3::normal(vnormal);
        let diffuse = self.color * self.kd * max(l.dot(&n), 0.0);
        let r = Vector3::reflect(&l, &n);
        let e = -Vector3::normal(neg_veye);
        let specular = *light_color * self.ks * max(e.dot(&r), 0.0).powf(self.exp as f32);
        diffuse + specular
    }

    /// Calculate the color for the given light source when hitting a point with this material with a ray
    pub fn get_color(&self, point: &Point3, normal: &Vector3, light: &Light, ray: &Ray) -> Color {
        match light {
            Light::Ambient { color: _ } => self.color * self.ka,
            Light::Parallel { color, direction } => self.phong(color, direction, normal, ray.dir()),
            Light::Point { color, position } => {
                let dir = *point - *position;
                self.phong(color, &dir, normal, ray.dir())
            }
        }
    }
}

/// Struct to represent an intersection of a ray and a sphere
/// has to live at least as long as the sphere, since it borrows its material
pub struct Intersection<'a> {
    pub point: Point3,
    pub t: f32,
    pub normal: Vector3,
    pub material: &'a Material,
}

impl Intersection<'_> {
    /// Calculate the color of the intersection point
    pub fn get_color(&self, light: &Light, ray: &Ray) -> Color {
        self.material
            .get_color(&self.point, &self.normal, light, ray)
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

        let behind = Ray::new(Point3::zero(), Vector3::new(0., 0., 1.));
        assert!(sphere.intersection(&behind).is_none())
    }
}
