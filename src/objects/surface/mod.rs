use crate::math::{Mat4, Point3, Ray, Vec3};
use crate::objects::surface::mesh::Mesh;
use crate::objects::surface::sphere::Sphere;

mod intersection;
mod material;
mod mesh;
mod sphere;

pub use intersection::Intersection;
pub use material::{Material, ShadingModel, Texture};
pub use mesh::Triangle;

type Texel = (f32, f32);

/// either a sphere or a mesh
#[derive(Debug)]
enum Object {
    Sphere(Sphere),
    Mesh(Box<Mesh>), // Box to keep the enum small
}

/// struct that bundles the (inverse) transformation
#[derive(Debug)]
struct Transform {
    transform: Mat4,
    normal_transform: Mat4,
}

/// struct to represent any surface in 3D
/// Either a `Sphere` or a `Mesh`
#[derive(Debug)]
pub struct Surface {
    obj: Object,
    transform: Option<Box<Transform>>,
}

impl Surface {
    /// Create a new sphere object from a radius and center
    pub fn sphere(center: Point3, radius: f32, material: Material) -> Surface {
        Surface {
            obj: Object::Sphere(Sphere::new(center, radius, material)),
            transform: None,
        }
    }

    /// Create a new mesh object from a triangle soup
    pub fn mesh(triangles: Vec<Triangle>, material: Material) -> Surface {
        Surface {
            obj: Object::Mesh(Box::new(Mesh::new(triangles, material))),
            transform: None,
        }
    }

    /// Determine if this surface intersects with the ray
    pub fn has_intersection(&self, with: &Ray) -> bool {
        let with = if let Some(t) = &self.transform {
            with.transform(&t.transform)
        } else {
            *with
        };

        match &self.obj {
            Object::Sphere(s) => s.has_intersection(&with),
            Object::Mesh(m) => m.has_intersection(&with),
        }
    }

    /// Calculate the intersection of the surface and the ray if it exists
    pub fn intersection(&self, with: &Ray) -> Option<Intersection> {
        let original_ray = with;
        let with = if let Some(t) = &self.transform {
            with.transform(&t.transform)
        } else {
            *with
        };

        let intersection = match &self.obj {
            Object::Sphere(s) => s.intersection(&with),
            Object::Mesh(m) => m.intersection(&with),
        }?;

        let normal = if let Some(t) = &self.transform {
            Vec3::normal(&t.normal_transform.transform_vector(&intersection.normal))
        } else {
            Vec3::normal(&intersection.normal)
        };

        Some(Intersection {
            point: original_ray.at(intersection.t)?,
            t: intersection.t,
            normal,
            texel: intersection.texel,
            material: intersection.material,
        })
    }

    /// set the transformation of the surface
    pub fn set_transform(&mut self, transform: Mat4, normal_transform: Mat4) {
        self.transform = Some(Box::new(Transform {
            transform,
            normal_transform,
        }))
    }
}
