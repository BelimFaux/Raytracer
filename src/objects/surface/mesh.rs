use crate::math::{Point3, Ray, Vector3};

use super::{Intersection, Material};

/// struct to represent a triangle in 3D-Space
#[derive(Debug, PartialEq)]
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
    fn normal_at(&self, _u: f32, _v: f32) -> Vector3 {
        (self.normals[0] + self.normals[1] + self.normals[2]) / 3.
    }

    /// Test if the triangle intersects with the ray
    /// using the [Moeller-Trombore algorithm](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html)
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

        with.t_in_range(t)
    }

    /// Calculates the intersection of the triangle and the `with` Ray if present
    /// using the [Moeller-Trombore algorithm](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html)
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

        if with.t_in_range(t) {
            Some((self.normal_at(u, v), t))
        } else {
            None
        }
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
    /// Create a new mesh
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Mesh {
        Mesh {
            triangles,
            material,
        }
    }

    /// Test if the mesh intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        self.triangles.iter().any(|t| t.has_intersection(with))
    }

    /// Calculates the intersection of the mesh and the `with` Ray if present
    /// Returns `None` if there is no intersection
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

#[cfg(test)]
mod tests {
    use crate::math::Vector3;

    use super::*;

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
