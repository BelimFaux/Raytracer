use core::f32;

use crate::{
    math::{lerp, min, Point3, Quat, Ray, Vec3},
    objects::surface::Texel,
};

#[derive(Debug)]
struct Animation {
    startc: Quat,
    endc: Option<Quat>,
}

/// Struct to represent a ray-tracable 4d julia set
#[derive(Debug)]
pub struct JuliaSet {
    pos: Point3,
    c: Quat,
    max_iterations: u32,
    epsilon: f32,
    animation: Box<Animation>,
}

impl JuliaSet {
    /// constants taken from [this paper](https://www.cs.cmu.edu/~kmcrane/Projects/QuaternionJulia/paper.pdf)
    const BOUNDING_RADIUS_2: f32 = 2.5;
    const ESCAPE_THRESHOLD: f32 = 1e1;
    const DEL: f32 = 1e-4;

    /// Create a new julia set
    pub fn new(pos: Point3, c: Quat, max_iterations: u32, epsilon: f32) -> JuliaSet {
        JuliaSet {
            pos,
            c,
            max_iterations,
            epsilon,
            animation: Box::new(Animation {
                startc: c,
                endc: None,
            }),
        }
    }

    /// Set the endconstant
    pub fn set_end(&mut self, ec: Quat) {
        self.animation.endc = Some(ec);
    }

    /// set the frame percentage the lerp between starting and ending constant
    pub fn set_frame(&mut self, w: f32) {
        if let Some(ec) = self.animation.endc {
            self.c = lerp(self.animation.startc, ec, w);
        }
    }

    /// iterate the given quaternion to find the intersection in the julia set
    /// taken from [this paper](https://www.cs.cmu.edu/~kmcrane/Projects/QuaternionJulia/paper.pdf)
    fn iterate_intersect(&self, q: &mut Quat) -> Quat {
        let mut qp = Quat::new(1., 0., 0., 0.);
        for _ in 0..self.max_iterations {
            qp = (&*q * &qp) * 2.;
            *q = q.square() + self.c;

            if q.length_squared() > Self::ESCAPE_THRESHOLD {
                break;
            }
        }

        qp
    }

    /// Calculate the distance to the intersection point with the julia set
    /// No intersection, if the distance is smaller than the epsilon
    /// taken from [this paper](https://www.cs.cmu.edu/~kmcrane/Projects/QuaternionJulia/paper.pdf)
    fn intersection_dist(&self, with: &Ray) -> (f32, Point3) {
        let mut dist;
        let mut orig = *with.orig();
        let dir = *with.dir();
        loop {
            let mut z = Quat::new(orig[0], orig[1], orig[2], 0.);
            let zp = self.iterate_intersect(&mut z);

            let norm_z = z.length();
            dist = 0.5 * norm_z * norm_z.log2() / zp.length();

            orig += dir * dist;

            if dist < self.epsilon || orig.length_squared() > Self::BOUNDING_RADIUS_2 {
                break;
            }
        }

        (dist, orig)
    }

    /// Calculate the intersection with the bounding sphere
    /// doesn't use the sphere struct, since radius is constant and center is at 0
    fn sphere_intersect(with: &Ray) -> Option<f32> {
        let a = with.dir().length_squared();
        let h = with.dir().dot(with.orig());
        let c = with.orig().length_squared() - Self::BOUNDING_RADIUS_2;
        let discr = h * h - a * c;
        if discr < 0. {
            return None;
        }
        let discr = discr.sqrt();
        Some(min(-h + discr, -h - discr) / a)
    }

    /// Normal estimation for point on a julia set
    /// taken from [this paper](https://www.cs.cmu.edu/~kmcrane/Projects/QuaternionJulia/paper.pdf)
    fn estimate_normal(&self, p: Point3) -> Vec3 {
        let qp = Quat::new(p[0], p[1], p[2], 0.);

        let mut gx1 = qp - Quat::new(Self::DEL, 0., 0., 0.);
        let mut gx2 = qp + Quat::new(Self::DEL, 0., 0., 0.);
        let mut gy1 = qp - Quat::new(0., Self::DEL, 0., 0.);
        let mut gy2 = qp + Quat::new(0., Self::DEL, 0., 0.);
        let mut gz1 = qp - Quat::new(0., 0., Self::DEL, 0.);
        let mut gz2 = qp + Quat::new(0., 0., Self::DEL, 0.);

        for _ in 0..self.max_iterations {
            gx1 = gx1.square() + self.c;
            gx2 = gx2.square() + self.c;
            gy1 = gy1.square() + self.c;
            gy2 = gy2.square() + self.c;
            gz1 = gz1.square() + self.c;
            gz2 = gz2.square() + self.c;
        }

        Vec3::normal(&Vec3::new(
            gx2.length() - gx1.length(),
            gy2.length() - gy1.length(),
            gz2.length() - gz1.length(),
        ))
    }

    pub fn has_intersection(&self, with: &Ray) -> bool {
        let with = Ray::new(*with.orig() - self.pos, *with.dir());
        let t = if let Some(t) = Self::sphere_intersect(&with) {
            t
        } else {
            return false;
        };
        let p = if let Some(p) = with.at(t) {
            p
        } else {
            return false;
        };
        let r = Ray::new(p, *with.dir());
        let (dist, _) = self.intersection_dist(&r);

        dist < self.epsilon
    }

    /// Calculate the nearest intersection point with the julia set
    /// Most calculations are taken from [this paper](https://www.cs.cmu.edu/~kmcrane/Projects/QuaternionJulia/paper.pdf)
    pub fn intersection(&self, with: &Ray) -> Option<(f32, Vec3, Texel)> {
        let with = Ray::new(*with.orig() - self.pos, *with.dir());
        let t = Self::sphere_intersect(&with)?;
        let r = Ray::new(with.at(t)?, *with.dir());
        let (dist, p) = self.intersection_dist(&r);

        if dist >= self.epsilon {
            return None;
        }

        Some((t + dist, self.estimate_normal(p), (0., 0.)))
    }
}
