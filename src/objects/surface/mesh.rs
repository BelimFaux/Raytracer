use crate::math::{max, min, Point3, Ray, Vec3};

use super::Texel;

/// struct to represent a triangle in 3D-Space
#[derive(Debug, PartialEq)]
pub struct Triangle {
    points: [Point3; 3],
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

/// Axis-aligned bounding box (AABB)
#[derive(Clone, Debug)]
struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl BoundingBox {
    /// Constructs a bounding box that encapsulates all given points
    pub fn from(points: &[Point3]) -> BoundingBox {
        let cmp_f32 =
            |lhs: &f32, rhs: &f32| lhs.partial_cmp(rhs).expect("Points should not contain NaN");

        let min_x = points.iter().map(|p| p[0]).min_by(cmp_f32).unwrap_or(0.);
        let max_x = points.iter().map(|p| p[0]).max_by(cmp_f32).unwrap_or(0.);
        let min_y = points.iter().map(|p| p[1]).min_by(cmp_f32).unwrap_or(0.);
        let max_y = points.iter().map(|p| p[1]).max_by(cmp_f32).unwrap_or(0.);
        let min_z = points.iter().map(|p| p[2]).min_by(cmp_f32).unwrap_or(0.);
        let max_z = points.iter().map(|p| p[2]).max_by(cmp_f32).unwrap_or(0.);

        BoundingBox {
            min: Vec3::new(min_x, min_y, min_z),
            max: Vec3::new(max_x, max_y, max_z),
        }
    }

    /// Determine if bounding box intersects with the ray
    /// using [Smits method](https://people.csail.mit.edu/amy/papers/box-jgt.pdf)
    #[allow(clippy::similar_names)]
    pub fn has_intersection(&self, with: &Ray) -> bool {
        let (tmin, tmax) = if with.dir()[0] >= 0. {
            (
                (self.min[0] - with.orig()[0]) / with.dir()[0],
                (self.max[0] - with.orig()[0]) / with.dir()[0],
            )
        } else {
            (
                (self.max[0] - with.orig()[0]) / with.dir()[0],
                (self.min[0] - with.orig()[0]) / with.dir()[0],
            )
        };

        let (tymin, tymax) = if with.dir()[1] >= 0. {
            (
                (self.min[1] - with.orig()[1]) / with.dir()[1],
                (self.max[1] - with.orig()[1]) / with.dir()[1],
            )
        } else {
            (
                (self.max[1] - with.orig()[1]) / with.dir()[1],
                (self.min[1] - with.orig()[1]) / with.dir()[1],
            )
        };

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        let tmin = max(tmin, tymin);
        let tmax = min(tmax, tymax);

        let (tzmin, tzmax) = if with.dir()[2] >= 0. {
            (
                (self.min[2] - with.orig()[2]) / with.dir()[2],
                (self.max[2] - with.orig()[2]) / with.dir()[2],
            )
        } else {
            (
                (self.max[2] - with.orig()[2]) / with.dir()[2],
                (self.min[2] - with.orig()[2]) / with.dir()[2],
            )
        };

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        let tmin = max(tmin, tzmin);
        let tmax = min(tmax, tzmax);

        (tmin < with.max_t()) && (tmax > 0.)
    }
}

/// struct to represent a mesh in a 3D-Space
/// Holds a Triangle 'soup' and material
/// also contains a bounding box to speed up intersection tests
#[derive(Debug)]
pub(super) struct Mesh {
    triangles: Vec<Triangle>,
    bounding_box: BoundingBox,
}

impl Mesh {
    /// Create a new mesh
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        let bounding_box = BoundingBox::from(
            &triangles
                .iter()
                .flat_map(|tri| tri.points)
                .collect::<Vec<_>>(),
        );
        Mesh {
            triangles,
            bounding_box,
        }
    }

    /// Test if the mesh intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        if self.bounding_box.has_intersection(with) {
            self.triangles.iter().any(|t| t.has_intersection(with))
        } else {
            false
        }
    }

    /// Calculates the intersection of the mesh and the `with` Ray if present
    /// Returns `None` if there is no intersection
    pub fn intersection(&self, with: &Ray) -> Option<(f32, Vec3, Texel)> {
        if !self.bounding_box.has_intersection(with) {
            return None;
        }

        let (normal, texel, t) = self
            .triangles
            .iter()
            .filter_map(|t| t.intersection(with))
            .min_by(|lhs, rhs| lhs.2.partial_cmp(&rhs.2).expect("t should not be NaN"))?;

        Some((t, normal, texel))
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

    #[test]
    fn construct_bounding_box() {
        let points = vec![
            Point3::new(-1., 0., -1.),
            Point3::new(1., 0., -1.),
            Point3::new(0., 1., -1.),
        ];

        let aabb = BoundingBox::from(&points);

        assert_eq!(aabb.min, Vec3::new(-1., 0., -1.));
        assert_eq!(aabb.max, Vec3::new(1., 1., -1.));
    }

    #[test]
    fn intersect_bounding_box() {
        let points = vec![
            Point3::new(-1., 0., -1.),
            Point3::new(1., 0., -1.),
            Point3::new(0., 1., -1.),
            Point3::new(-1., 0., 0.),
            Point3::new(1., 0., 0.),
            Point3::new(0., 1., 0.),
        ];

        let aabb = BoundingBox::from(&points);

        let hit = Ray::new(Point3::zero(), Vec3::new(0., 0., -1.));
        assert!(aabb.has_intersection(&hit));

        let no_hit = Ray::new(Point3::zero(), Vec3::new(0., 1., 1.));
        assert!(!aabb.has_intersection(&no_hit));
    }
}
