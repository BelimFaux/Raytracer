use crate::{
    math::{Ray, Vec3},
    objects::{surface::Texel, BoundingBox, Triangle},
};

#[derive(Debug, Copy, Clone)]
struct BVHTriangle {
    index: usize,
    centroid: Vec3,
    bound: BoundingBox,
}

impl BVHTriangle {
    fn from(triangle: &Triangle, index: usize) -> BVHTriangle {
        let aabb = BoundingBox::from(&triangle.points);
        BVHTriangle {
            index,
            centroid: aabb.centroid(),
            bound: aabb,
        }
    }
}

#[derive(Debug)]
enum BVHNode {
    LeafNode {
        aabb: BoundingBox,
        triangles: Vec<BVHTriangle>,
    },
    InternalNode {
        aabb: BoundingBox,
        left_child: Box<BVHNode>,
        right_child: Box<BVHNode>,
    },
}

impl BVHNode {
    const ACTIVE_TRIANGLE_COUNT: usize = 2;

    fn split_sorted_median(
        triangles: &mut [BVHTriangle],
        axis: usize,
    ) -> (&mut [BVHTriangle], &mut [BVHTriangle]) {
        let mid = triangles.len() / 2;
        let _ = triangles.select_nth_unstable_by(mid, |lhs, rhs| {
            lhs.centroid[axis]
                .partial_cmp(&rhs.centroid[axis])
                .expect("Points should not contain NaN")
        });
        triangles.split_at_mut(mid)
    }

    pub fn build_bvh(triangles: &mut [BVHTriangle]) -> BVHNode {
        let aabb = BoundingBox::union(triangles.iter().map(|tri| tri.bound));
        if triangles.len() < Self::ACTIVE_TRIANGLE_COUNT {
            return BVHNode::LeafNode {
                aabb,
                triangles: triangles.to_vec(),
            };
        }

        let (left_part, right_part) = Self::split_sorted_median(triangles, aabb.longest_extend());
        BVHNode::InternalNode {
            aabb,
            left_child: Box::new(Self::build_bvh(left_part)),
            right_child: Box::new(Self::build_bvh(right_part)),
        }
    }

    pub fn has_intersection(&self, with: &Ray, orig_triangles: &[Triangle]) -> bool {
        match self {
            BVHNode::LeafNode { aabb, triangles } => {
                if !aabb.has_intersection(with) {
                    return false;
                }
                triangles
                    .iter()
                    .any(|t| orig_triangles[t.index].has_intersection(with))
            }
            BVHNode::InternalNode {
                aabb,
                left_child,
                right_child,
            } => {
                if !aabb.has_intersection(with) {
                    return false;
                }
                left_child.has_intersection(with, orig_triangles)
                    || right_child.has_intersection(with, orig_triangles)
            }
        }
    }

    pub fn intersection(
        &self,
        with: &Ray,
        orig_triangles: &[Triangle],
    ) -> Option<(f32, Vec3, Texel)> {
        match self {
            BVHNode::LeafNode { aabb, triangles } => {
                if !aabb.has_intersection(with) {
                    return None;
                }
                let (normal, texel, t) = triangles
                    .iter()
                    .filter_map(|t| orig_triangles[t.index].intersection(with))
                    .min_by(|lhs, rhs| lhs.2.partial_cmp(&rhs.2).expect("t should not be NaN"))?;

                Some((t, normal, texel))
            }
            BVHNode::InternalNode {
                aabb,
                left_child,
                right_child,
            } => {
                if !aabb.has_intersection(with) {
                    return None;
                }
                let l_opt = left_child.intersection(with, orig_triangles);
                let r_opt = right_child.intersection(with, orig_triangles);

                match (l_opt, r_opt) {
                    (Some(l), Some(r)) => {
                        if l.0 < r.0 {
                            l_opt
                        } else {
                            r_opt
                        }
                    }
                    (Some(_), None) => l_opt,
                    (None, Some(_)) => r_opt,
                    (None, None) => None,
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Bvh {
    root: BVHNode,
    triangles: Vec<Triangle>,
}

impl Bvh {
    #[must_use]
    pub fn new(triangles: Vec<Triangle>) -> Bvh {
        let mut triangle_info: Vec<_> = triangles
            .iter()
            .enumerate()
            .map(|(i, tri)| BVHTriangle::from(tri, i))
            .collect();
        Bvh {
            root: BVHNode::build_bvh(&mut triangle_info),
            triangles,
        }
    }

    #[must_use]
    pub fn has_intersection(&self, with: &Ray) -> bool {
        self.root.has_intersection(with, &self.triangles)
    }

    pub fn intersection(&self, with: &Ray) -> Option<(f32, Vec3, Texel)> {
        self.root.intersection(with, &self.triangles)
    }
}
