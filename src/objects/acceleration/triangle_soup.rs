use crate::{
    math::{Ray, Vec3},
    objects::{surface::Texel, BoundingBox, Triangle},
};

#[derive(Debug)]
pub struct TriangleSoup {
    triangles: Vec<Triangle>,
}

impl TriangleSoup {
    #[must_use]
    pub fn new(triangles: Vec<Triangle>) -> TriangleSoup {
        TriangleSoup { triangles }
    }

    #[must_use]
    pub fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox::from(
            &self
                .triangles
                .iter()
                .flat_map(|tri| tri.points)
                .collect::<Vec<_>>(),
        )
    }

    #[must_use]
    pub fn has_intersection(&self, with: &Ray) -> bool {
        self.triangles.iter().any(|t| t.has_intersection(with))
    }

    #[must_use]
    /// Get the closest intersection with any triangle if one exists
    ///
    /// # Panics
    ///
    /// if any of the points contain a NaN value
    pub fn intersection(&self, with: &Ray) -> Option<(f32, Vec3, Texel)> {
        let (normal, texel, t) = self
            .triangles
            .iter()
            .filter_map(|t| t.intersection(with))
            .min_by(|lhs, rhs| lhs.2.partial_cmp(&rhs.2).expect("t should not be NaN"))?;

        Some((t, normal, texel))
    }
}
