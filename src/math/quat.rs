use std::ops;

use crate::math::Vec3;

/// Struct to represent a quaternion
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    r: f32,
    v: Vec3,
}

impl Quat {
    /// Create a new quaternion
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quat {
        Quat {
            r: x,
            v: Vec3::new(y, z, w),
        }
    }

    /// Computes the square of the quaternion
    /// Same as `q * q` but more efficient
    pub fn square(&self) -> Quat {
        Quat {
            r: self.r * self.r - self.v.length_squared(),
            v: 2. * self.r * self.v,
        }
    }

    /// Compute the squared length of the quaternion
    pub fn length_squared(&self) -> f32 {
        self.r * self.r + self.v.length_squared()
    }

    /// Compute the length of the quaternion
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl ops::Add for Quat {
    type Output = Quat;

    fn add(self, rhs: Self) -> Self::Output {
        Quat {
            r: self.r + rhs.r,
            v: self.v + rhs.v,
        }
    }
}

impl ops::Sub for Quat {
    type Output = Quat;

    fn sub(self, rhs: Self) -> Self::Output {
        Quat {
            r: self.r - rhs.r,
            v: self.v - rhs.v,
        }
    }
}

impl ops::Mul for &Quat {
    type Output = Quat;

    /// Multiplication for quaternions
    /// generally not commutative
    fn mul(self, rhs: Self) -> Self::Output {
        Quat {
            r: self.r * rhs.r - self.v.dot(&rhs.v),
            v: self.r * rhs.v + rhs.r * self.v + self.v.cross(&rhs.v),
        }
    }
}

impl ops::Mul<f32> for Quat {
    type Output = Quat;

    /// Multiplication for quaternions
    /// generally not commutative
    fn mul(self, rhs: f32) -> Self::Output {
        Quat {
            r: self.r * rhs,
            v: self.v * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quat_multiplication() {
        let lhs = Quat::new(1., 2., 3., 4.);
        let rhs = Quat::new(5., 6., 7., 8.);

        let expected = Quat::new(-60., 12., 30., 24.);

        assert_eq!(expected, &lhs * &rhs);
    }

    #[test]
    fn quat_square() {
        let q = Quat::new(1., 2., 3., 4.);

        let expected = Quat::new(-28., 4., 6., 8.);

        assert_eq!(expected, q.square());
    }
}
