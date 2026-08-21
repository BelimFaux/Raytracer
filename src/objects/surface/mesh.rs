use crate::{
    math::{Point3, Ray, Vec3},
    objects::acceleration::Bvh,
};

use super::Texel;

/// struct to represent a triangle in 3D-Space
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Triangle {
    pub points: [Point3; 3],
    normals: [Vec3; 3],
    texcoords: [Texel; 3],
}

impl Triangle {
    const INTERSECT_EPS: f32 = 1e-8;

    /// Create a new triangle from the edge points and the corresponding normals
    /// The normals and the points should be in the same order in the arrays
    #[must_use]
    pub fn new(points: [Point3; 3], normals: [Vec3; 3], texcoords: [Texel; 3]) -> Triangle {
        Triangle {
            points,
            normals,
            texcoords,
        }
    }

    /// Return the normal for the given barycentric coordinates
    fn normal_at(&self, a: f32, b: f32) -> Vec3 {
        (1. - a - b) * self.normals[0] + a * self.normals[1] + b * self.normals[2]
    }

    /// Return the texel at the given barycentric coordinates
    fn texel_at(&self, a: f32, b: f32) -> (f32, f32) {
        let t = self.texcoords;
        (
            ((1. - a - b) * t[0].0 + a * t[1].0 + b * t[2].0) % 1.,
            ((1. - a - b) * t[0].1 + a * t[1].1 + b * t[2].1) % 1.,
        )
    }

    /// Test if the triangle intersects with the ray
    /// using the [Moeller-Trombore algorithm](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html)
    #[must_use]
    pub fn has_intersection(&self, with: &Ray) -> bool {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let dxe2 = with.dir().cross(&e2);
        let det = e1.dot(&dxe2);

        if det.abs() < Self::INTERSECT_EPS {
            return false;
        }

        let inv_det = 1. / det;

        let s = *with.orig() - self.points[0];
        let a = s.dot(&dxe2) * inv_det;
        if !(0. ..=1.).contains(&a) {
            return false;
        }

        let sxe1 = s.cross(&e1);
        let b = with.dir().dot(&sxe1) * inv_det;
        if b < 0. || a + b > 1. {
            return false;
        }

        let t = e2.dot(&sxe1) * inv_det;

        with.t_in_range(t)
    }

    /// Calculates the normal, the texel and the t value of the triangle and the `with` Ray if present
    /// using the [Moeller-Trombore algorithm](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection.html)
    /// Returns `None` if there is no intersection
    #[must_use]
    pub fn intersection(&self, with: &Ray) -> Option<(Vec3, Texel, f32)> {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let dxe2 = with.dir().cross(&e2);
        let det = e1.dot(&dxe2);

        if det.abs() < Self::INTERSECT_EPS {
            return None;
        }

        let inv_det = 1. / det;

        let s = *with.orig() - self.points[0];
        let a = s.dot(&dxe2) * inv_det;
        if !(0. ..=1.).contains(&a) {
            return None;
        }

        let sxe1 = s.cross(&e1);
        let b = with.dir().dot(&sxe1) * inv_det;
        if b < 0. || a + b > 1. {
            return None;
        }

        let t = e2.dot(&sxe1) * inv_det;

        if with.t_in_range(t) {
            Some((self.normal_at(a, b), self.texel_at(a, b), t))
        } else {
            None
        }
    }
}

/// struct to represent a mesh in a 3D-Space
/// Holds a Triangle 'soup' and material
/// also contains a bounding box to speed up intersection tests
#[derive(Debug)]
pub(super) struct Mesh {
    triangles: Bvh,
}

impl Mesh {
    /// Create a new mesh
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        Mesh {
            triangles: Bvh::new(triangles),
        }
    }

    /// Test if the mesh intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        self.triangles.has_intersection(with)
    }

    /// Calculates the intersection of the mesh and the `with` Ray if present
    /// Returns `None` if there is no intersection
    pub fn intersection(&self, with: &Ray) -> Option<(f32, Vec3, Texel)> {
        self.triangles.intersection(with)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3;

    use super::*;

    #[test]
    fn triangle_intersection_test() {
        let triangle = Triangle::new(
            [
                Point3::new(-1., 0., -1.),
                Point3::new(1., 0., -1.),
                Point3::new(0., 1., -1.),
            ],
            [Vec3::zero(); 3],
            [(0., 0.); 3],
        );

        // should hit the triangle at point (0, 0, -1)
        let hit = Ray::new(Point3::zero(), Vec3::new(0., 0., -1.));
        assert!(triangle.has_intersection(&hit));
        assert!(triangle
            .intersection(&hit)
            .is_some_and(|(_, _, t)| (t - 1.).abs() < f32::EPSILON));

        let no_hit = Ray::new(Point3::zero(), Vec3::new(0., 1., 1.));
        assert!(!triangle.has_intersection(&no_hit));
        assert!(triangle.intersection(&no_hit).is_none());
    }
}
