use crate::math::{Point3, Ray, Vector3};

use super::{Intersection, Material, Texel};

/// struct to represent a triangle in 3D-Space
#[derive(Debug, PartialEq)]
pub struct Triangle {
    points: [Point3; 3],
    normals: [Vector3; 3],
    texcoords: [Texel; 3],
}

impl Triangle {
    const INTERSECT_EPS: f32 = 1e-8;

    /// Create a new triangle from the edge points and the corresponding normals
    /// The normals and the points should be in the same order in the arrays
    pub fn new(points: [Point3; 3], normals: [Vector3; 3], texcoords: [Texel; 3]) -> Triangle {
        Triangle {
            points,
            normals,
            texcoords,
        }
    }

    /// Return the normal for the given barycentric coordinates
    fn normal_at(&self, a: f32, b: f32) -> Vector3 {
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
    pub fn intersection(&self, with: &Ray) -> Option<(Vector3, Texel, f32)> {
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
    min: Vector3,
    max: Vector3,
}

impl BoundingBox {
    /// Constructs a bounding box that encapsulates all given points
    pub fn from(points: Vec<Point3>) -> BoundingBox {
        let cmp_f32 =
            |lhs: &f32, rhs: &f32| lhs.partial_cmp(rhs).expect("Points should not contain NaN");

        let min_x = points.iter().map(|p| p[0]).min_by(cmp_f32).unwrap_or(0.);
        let max_x = points.iter().map(|p| p[0]).max_by(cmp_f32).unwrap_or(0.);
        let min_y = points.iter().map(|p| p[1]).min_by(cmp_f32).unwrap_or(0.);
        let max_y = points.iter().map(|p| p[1]).max_by(cmp_f32).unwrap_or(0.);
        let min_z = points.iter().map(|p| p[2]).min_by(cmp_f32).unwrap_or(0.);
        let max_z = points.iter().map(|p| p[2]).max_by(cmp_f32).unwrap_or(0.);

        BoundingBox {
            min: Vector3::new(min_x, min_y, min_z),
            max: Vector3::new(max_x, max_y, max_z),
        }
    }

    /// Determine if bounding box intersects with the ray
    /// using Andrew Kensler's [ray-AABB algorithm](https://psgraphics.blogspot.com/2016/02/new-simple-ray-box-test-from-andrew.html)
    /// will fail if any components of the AABBs min or max vectors are NaN
    pub fn has_intersection(&self, with: &Ray) -> bool {
        for a in 0..3 {
            let inv_d = 1. / with.dir()[a];
            let mut t0 = (self.min[a] - with.orig()[a]) * inv_d;
            let mut t1 = (self.max[a] - with.orig()[a]) * inv_d;

            if inv_d < 0. {
                (t0, t1) = (t1, t0);
            }

            let tmin = if t0 > 0. { t0 } else { 0.0 };
            let tmax = if t1 < with.max_t() { t1 } else { with.max_t() };

            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}

/// struct to represent a mesh in a 3D-Space
/// Holds a Triangel 'soup' and material
/// also contains a bounding box to speed up intersection tests
#[derive(Debug)]
pub struct Mesh {
    triangles: Vec<Triangle>,
    material: Material,
    bounding_box: BoundingBox,
}

impl Mesh {
    /// Create a new mesh
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Mesh {
        let bounding_box = BoundingBox::from(triangles.iter().flat_map(|tri| tri.points).collect());
        Mesh {
            triangles,
            material,
            bounding_box,
        }
    }

    /// Test if the mesh intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        if !self.bounding_box.has_intersection(with) {
            false
        } else {
            self.triangles.iter().any(|t| t.has_intersection(with))
        }
    }

    /// Calculates the intersection of the mesh and the `with` Ray if present
    /// Returns `None` if there is no intersection
    pub fn intersection(&self, with: &Ray) -> Option<Intersection> {
        if !self.bounding_box.has_intersection(with) {
            return None;
        }

        let (normal, texel, t) = self
            .triangles
            .iter()
            .filter_map(|t| t.intersection(with))
            .min_by(|lhs, rhs| lhs.2.partial_cmp(&rhs.2).expect("t should not be NaN"))?;

        Some(Intersection {
            point: with.at(t)?,
            t,
            normal,
            texel,
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
            [(0., 0.); 3],
        );

        // should hit the triangle at point (0, 0, -1)
        let hit = Ray::new(Point3::zero(), Vector3::new(0., 0., -1.));
        assert!(triangle.has_intersection(&hit));
        assert!(triangle.intersection(&hit).is_some_and(|(_, _, t)| t == 1.));

        let no_hit = Ray::new(Point3::zero(), Vector3::new(0., 1., 1.));
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

        let aabb = BoundingBox::from(points);

        assert_eq!(aabb.min, Vector3::new(-1., 0., -1.));
        assert_eq!(aabb.max, Vector3::new(1., 1., -1.));
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

        let aabb = BoundingBox::from(points);

        let hit = Ray::new(Point3::zero(), Vector3::new(0., 0., -1.));
        assert!(aabb.has_intersection(&hit));

        let no_hit = Ray::new(Point3::zero(), Vector3::new(0., 1., 1.));
        assert!(!aabb.has_intersection(&no_hit));
    }
}
