use crate::math::Ray;

mod intersection;
mod material;
mod mesh;
mod sphere;

pub use intersection::Intersection;
pub use material::Material;
pub use mesh::{Mesh, Triangle};
pub use sphere::Sphere;

/// struct to represent any surface in 3D
/// Either a `Sphere` or a `Mesh`
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
