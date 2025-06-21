use crate::image;
use serde::Deserialize;
use std::ops;

/// Struct to represent a 3D-Vector
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub struct Vector3 {
    #[serde(rename = "@x", alias = "@r")]
    x: f32,
    #[serde(rename = "@y", alias = "@g")]
    y: f32,
    #[serde(rename = "@z", alias = "@b")]
    z: f32,
}

/// A point in 3D space
pub type Point3 = Vector3;

/// A color value with 3 floats representing red, green and blue
pub type Color = Vector3;

impl Color {
    /// Convert a color with values in range 0 to 1 to an RGB value with values from 0 to 255
    /// The components get clamped at 0 and 1
    pub fn to_rgb(self) -> image::Rgb {
        let r = (255.999 * self.x.clamp(0.0, 1.0)) as u8;
        let g = (255.999 * self.y.clamp(0.0, 1.0)) as u8;
        let b = (255.999 * self.z.clamp(0.0, 1.0)) as u8;
        [r, g, b]
    }
}

impl Vector3 {
    /// Create a new Vector from 3 floats
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Create a normal from a vector
    pub fn normal(from: &Vector3) -> Vector3 {
        let length = from.length();
        Vector3 {
            x: from.x / length,
            y: from.y / length,
            z: from.z / length,
        }
    }

    /// Creates a Vector with all components = 0
    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    /// computes the dot product
    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// computes the cross product of self x rhs (not commutative)
    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// returnes the length of the vector
    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    /// returnes the square of the length of the vector
    /// more efficient for comparisons
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// normalize the vector
    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// calculate the reflection direction from the given incident vector `i` and the normal `n`
    /// `i - 2.0 * dot(n, i) * n`
    pub fn reflect(i: &Vector3, n: &Vector3) -> Vector3 {
        *i - 2.0 * n.dot(i) * *n
    }
}

impl ops::Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bounds access"),
        }
    }
}

// --- Operators ---

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}

impl ops::Div<Vector3> for f32 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        rhs / self
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1. / rhs;
    }
}

// --- Tests ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_add_2_vectors() {
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(4., 5., 6.);

        let res = v1 + v2;
        let expected = Vector3::new(5., 7., 9.);

        assert_eq!(res, expected);

        let zero = Vector3::zero();

        assert_eq!(expected + zero, expected);

        // assign
        let mut v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(4., 5., 6.);
        v1 += v2;

        let expected = Vector3::new(5., 7., 9.);

        assert_eq!(v1, expected);
    }

    #[test]
    fn vector_subtract_2_vectors() {
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(4., 5., 6.);

        let res = v2 - v1;
        let expected = Vector3::new(3., 3., 3.);

        assert_eq!(res, expected);

        let zero = Vector3::zero();

        assert_eq!(expected - zero, expected);

        // assign
        let v1 = Vector3::new(1., 2., 3.);
        let mut v2 = Vector3::new(4., 5., 6.);
        v2 -= v1;

        let expected = Vector3::new(3., 3., 3.);

        assert_eq!(v2, expected);
    }

    #[test]
    fn vector_mul_and_div() {
        let v1 = Vector3::new(1., 2., 3.);

        let res = Vector3::new(2., 4., 6.);

        assert_eq!(v1 * 2., res);
        assert_eq!(v1 * 2., 2. * v1);

        assert_eq!(res / 2., v1);
        assert_eq!(res / 2., 2. / res);

        let mut v2 = Vector3::new(4., 5., 6.);
        v2 *= 2.;

        let res = Vector3::new(8., 10., 12.);
        assert_eq!(v2, res);

        v2 /= 2.;

        assert_eq!(v2, res / 2.);
    }

    #[test]
    fn vector_dot_and_cross() {
        let v1 = Vector3::new(1., 2., 3.);
        let v2 = Vector3::new(4., 5., 6.);

        let exp_dot = 32.;

        assert_eq!(v1.dot(&v2), exp_dot);
        assert_eq!(v2.dot(&v1), exp_dot);

        let exp_cross = Vector3::new(-3., 6., -3.);

        assert_eq!(v1.cross(&v2), exp_cross);
        assert_eq!(v2.cross(&v1), -exp_cross);
    }

    #[test]
    fn vector_length() {
        let mut v1 = Vector3::new(1., 2., 2.);

        assert_eq!(v1.length_squared(), 9.);
        assert_eq!(v1.length(), 3.);

        v1.normalize();

        assert_eq!(v1.length(), 1.);
    }

    #[test]
    fn convert_color_to_rgb() {
        let color = Color::new(1., 0.5, 0.); // Orange
        let pixel = color.to_rgb();

        let expected = [255, 127, 0];

        assert_eq!(pixel, expected);
    }
}
