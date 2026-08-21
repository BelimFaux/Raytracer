use crate::math::{max, min, Point3, Ray, Vec3};
/// Axis-aligned bounding box (AABB)
#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    min: Vec3,
    max: Vec3,
}

impl BoundingBox {
    /// Constructs a bounding box that encapsulates all given points
    ///
    /// # Panics
    ///
    /// If any of the points contain a NaN value
    #[must_use]
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

    pub fn union<TIter>(aabbs: TIter) -> BoundingBox
    where
        TIter: Iterator<Item = BoundingBox>,
    {
        let init = BoundingBox {
            min: Vec3::zero(),
            max: Vec3::zero(),
        };
        aabbs.fold(init, |mut acc, elem| {
            acc.min[0] = min(acc.min[0], elem.min[0]);
            acc.min[1] = min(acc.min[1], elem.min[1]);
            acc.min[2] = min(acc.min[2], elem.min[2]);
            acc.max[0] = max(acc.max[0], elem.max[0]);
            acc.max[1] = max(acc.max[1], elem.max[1]);
            acc.max[2] = max(acc.max[2], elem.max[2]);
            acc
        })
    }

    #[must_use]
    pub fn centroid(&self) -> Point3 {
        (self.min + self.max) * 0.5
    }

    #[must_use]
    pub fn longest_extend(&self) -> usize {
        let diag = self.max - self.min;
        if diag[0] >= diag[1] && diag[0] >= diag[2] {
            return 0;
        }
        if diag[1] >= diag[0] && diag[1] >= diag[2] {
            return 1;
        }
        2
    }

    /// Determine if bounding box intersects with the ray
    /// using [Smits method](https://people.csail.mit.edu/amy/papers/box-jgt.pdf)
    #[allow(clippy::similar_names)]
    #[must_use]
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

#[cfg(test)]
mod tests {
    use super::*;

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
