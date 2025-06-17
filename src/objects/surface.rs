use crate::math::{max, Color, Point3, Ray, Vector3};

use super::Light;

#[derive(Debug)]
pub enum Surface {
    Sphere(Sphere),
    Mesh(Mesh),
}

impl Surface {
    pub fn has_intersection(&self, with: &Ray) -> bool {
        match self {
            Self::Sphere(s) => s.has_intersection(with),
            Self::Mesh(m) => m.has_intersection(with),
        }
    }

    pub fn intersection(&self, with: &Ray) -> Option<Intersection> {
        match self {
            Self::Sphere(s) => s.intersection(with),
            Self::Mesh(m) => m.intersection(with),
        }
    }
}

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

        let t = -b - h.sqrt();
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

/// struct to represent a triangle in 3D-Space
#[derive(Debug)]
pub struct Triangle {
    points: [Point3; 3],
    normals: [Vector3; 3],
}

impl Triangle {
    const INTERSECT_EPS: f32 = 1e-8;

    /// Create a new triangle from the edge points and the corresponding normals
    /// The normals and the points should be in the same order in the arrays
    pub fn new(points: [Point3; 3], normals: [Vector3; 3]) -> Triangle {
        Triangle { points, normals }
    }

    /// Return the normal for the given barycentric coordinates
    /// for flat shading this is constant
    pub fn normal_at(&self, _u: f32, _v: f32) -> Vector3 {
        (self.normals[0] + self.normals[1] + self.normals[2]) / 3.
    }

    /// Test if the triangle intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let pvec = with.dir().cross(&e2);
        let det = e1.dot(&pvec);

        if det.abs() < Self::INTERSECT_EPS {
            return false;
        }

        let inv_det = 1. / det;

        let tvec = *with.orig() - self.points[0];
        let u = tvec.dot(&pvec) * inv_det;
        if !(0. ..=1.).contains(&u) {
            return false;
        }

        let qvec = tvec.cross(&e1);
        let v = with.dir().dot(&qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return false;
        }

        let t = e2.dot(&qvec) * inv_det;

        t > 0.
    }

    /// Calculates the intersection of the triangle and the `with` Ray if present
    /// Returns `None` if there is no intersection
    pub fn intersection(&self, with: &Ray) -> Option<(Vector3, f32)> {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let pvec = with.dir().cross(&e2);
        let det = e1.dot(&pvec);

        if det.abs() < Self::INTERSECT_EPS {
            return None;
        }

        let inv_det = 1. / det;

        let tvec = *with.orig() - self.points[0];
        let u = tvec.dot(&pvec) * inv_det;
        if !(0. ..=1.).contains(&u) {
            return None;
        }

        let qvec = tvec.cross(&e1);
        let v = with.dir().dot(&qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = e2.dot(&qvec) * inv_det;

        Some((-self.normal_at(u, v), t))
    }
}

/// struct to represent a mesh in a 3D-Space
/// Holds a Triangel 'soup' and material
#[derive(Debug)]
pub struct Mesh {
    triangles: Vec<Triangle>,
    material: Material,
}

impl Mesh {
    pub fn new(_: Vec<Triangle>, material: Material) -> Mesh {
        /*
         * v 5.000000 7.499999 -10.000000
         * v 5.000000 -2.499999 -10.000000
         * v 5.000000 -2.499998 0.000000
         * v 5.000000 7.500001 -0.000002
         * v 5.000000 7.499999 -10.000000
         * v 5.000000 -2.499999 -10.000000
         * v -5.000000 -2.499998 -10.000000
         * v -4.999998 7.500001 -10.000000
         * v 5.000000 -2.500000 -0.000001
         * v 5.000000 -2.500000 -9.999999
         * v -5.000000 -2.500000 -9.999998
         * v -4.999998 -2.500000 0.000001
         * vn -1.000000 0.000000 0.000000
         * vn 0.000000 0.000000 1.000000
         * vn 0.000000 1.000000 0.000000
         */
        let p1 = Point3::new(5., 7.5, -10.);
        let p2 = Point3::new(5., -2.5, -10.);
        let p3 = Point3::new(5., -2.5, 0.);
        let p4 = Point3::new(5., 7.5, 0.);
        let p5 = Point3::new(5., 7.5, -10.);
        let p6 = Point3::new(5., -2.5, -10.);
        let p7 = Point3::new(-5., -2.5, -10.);
        let p8 = Point3::new(-5., 7.5, -10.);
        let p9 = Point3::new(5., -2.5, 0.);
        let p10 = Point3::new(5., -2.5, -10.);
        let p11 = Point3::new(-5., -2.5, -10.);
        let p12 = Point3::new(-5., -2.5, 0.);
        let n1 = Vector3::new(-1., 0., 0.);
        let n2 = Vector3::new(0., 0., 1.);
        let n3 = Vector3::new(0., 1., 0.);
        let triangles = vec![
            Triangle::new([p1, p4, p3], [n1; 3]),
            Triangle::new([p1, p3, p2], [n1; 3]),
            Triangle::new([p5, p8, p7], [n2; 3]),
            Triangle::new([p5, p7, p6], [n2; 3]),
            Triangle::new([p9, p10, p11], [n3; 3]),
            Triangle::new([p9, p11, p12], [n3; 3]),
        ];
        Mesh {
            triangles,
            material,
        }
    }

    pub fn has_intersection(&self, with: &Ray) -> bool {
        self.triangles.iter().any(|t| t.has_intersection(with))
    }

    pub fn intersection(&self, with: &Ray) -> Option<Intersection> {
        let (n, t) = self
            .triangles
            .iter()
            .filter_map(|t| t.intersection(with))
            .min_by(|lhs, rhs| lhs.1.partial_cmp(&rhs.1).expect("t should not be NaN"))?;
        Some(Intersection {
            point: with.at(t)?,
            t,
            normal: n,
            material: &self.material,
        })
    }
}

/// Struct to represent a Material
#[derive(Clone, Debug)]
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
            Light::Ambient { .. } => self.color * self.ka,
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

    #[test]
    fn triangle_intersection_test() {
        let triangle = Triangle::new(
            [
                Point3::new(-1., 0., -1.),
                Point3::new(1., 0., -1.),
                Point3::new(0., 1., -1.),
            ],
            [Vector3::zero(); 3],
        );

        // should hit the triangle at point (0, 0, -1)
        let hit = Ray::new(Point3::zero(), Vector3::new(0., 0., -1.));
        assert!(triangle.has_intersection(&hit));
        assert!(triangle.intersection(&hit).is_some_and(|(_, t)| t == 1.));

        let no_hit = Ray::new(Point3::zero(), Vector3::new(0., 1., 1.));
        assert!(!triangle.has_intersection(&no_hit));
        assert!(triangle.intersection(&no_hit).is_none());
    }
}
