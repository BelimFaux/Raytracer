use super::{Point3, Vec3};
use std::ops;

/// Struct to represent a 4D Matrix
#[derive(Debug, PartialEq)]
pub struct Mat4 {
    vals: [f32; 16],
}

impl Mat4 {
    /// create a mat4 look at function for camera transformations
    /// takes in the camera position `from`, the point to look at `at`, and the `up` vector
    #[inline]
    pub fn look_at(from: Point3, at: Point3, up: Vec3) -> Mat4 {
        let z = Vec3::normal(&(from - at));
        let x = Vec3::normal(&up.cross(&z));
        let y = Vec3::normal(&z.cross(&x));

        #[rustfmt::skip]
        let vals = [
            x[0], y[0], z[0], from[0],
            x[1], y[1], z[1], from[1],
            x[2], y[2], z[2], from[2],
              0.,   0.,   0.,      1.,
        ];

        Mat4 { vals }
    }

    /// Creates a matrix from a translation vector
    #[inline]
    pub fn from_translation(diff: Vec3) -> Mat4 {
        #[rustfmt::skip]
        let vals = [
            1., 0., 0., diff[0],
            0., 1., 0., diff[1],
            0., 0., 1., diff[2],
            0., 0., 0.,      1.,
        ];

        Mat4 { vals }
    }

    /// Create a matrix, that rotates around the x-axis by the given amount in radians
    #[inline]
    pub fn from_x_rotation(rad: f32) -> Mat4 {
        let sin_r = rad.sin();
        let cos_r = rad.cos();

        #[rustfmt::skip]
        let vals = [
            1.,    0.,     0., 0.,
            0., cos_r, -sin_r, 0.,
            0., sin_r,  cos_r, 0.,
            0.,    0.,     0., 1.,
        ];

        Mat4 { vals }
    }

    /// Create a matrix, that rotates around the y-axis by the given amount in radians
    #[inline]
    pub fn from_y_rotation(rad: f32) -> Mat4 {
        let sin_r = rad.sin();
        let cos_r = rad.cos();

        #[rustfmt::skip]
        let vals = [
             cos_r, 0., sin_r, 0.,
                0., 1.,    0., 0.,
            -sin_r, 0., cos_r, 0.,
                0., 0.,    0., 1.,
        ];

        Mat4 { vals }
    }

    /// Create a matrix, that rotates around the z-axis by the given amount in radians
    #[inline]
    pub fn from_z_rotation(rad: f32) -> Mat4 {
        let sin_r = rad.sin();
        let cos_r = rad.cos();

        #[rustfmt::skip]
        let vals = [
            cos_r, -sin_r, 0., 0.,
            sin_r,  cos_r, 0., 0.,
               0.,     0., 1., 0.,
               0.,     0., 0., 1.,
        ];

        Mat4 { vals }
    }

    /// Create a matrix that scales by the given amount in x, y, and z direction
    #[inline]
    pub fn from_scaling(scale: Vec3) -> Mat4 {
        let s = scale;

        #[rustfmt::skip]
        let vals = [
            s[0],   0.,   0., 0.,
              0., s[1],   0., 0.,
              0.,   0., s[2], 0.,
              0.,   0.,   0., 1.,
        ];

        Mat4 { vals }
    }
}

// -- Operators ---

impl ops::Mul<&Mat4> for &Vec3 {
    type Output = Vec3;

    /// Transform a vec3 with a mat4
    /// assumes w = 1 for the vector
    fn mul(self, rhs: &Mat4) -> Self::Output {
        let m = rhs.vals;
        let x = self[0];
        let y = self[1];
        let z = self[2];
        let w = m[12] * x + m[13] * y + m[14] * z + m[15];

        Vec3::new(
            (m[0] * x + m[1] * y + m[2] * z + m[3]) / w,
            (m[4] * x + m[5] * y + m[6] * z + m[7]) / w,
            (m[8] * x + m[9] * y + m[10] * z + m[11]) / w,
        )
    }
}

impl ops::Mul<&Mat4> for &Mat4 {
    type Output = Mat4;

    /// Regular matrix multiplication
    fn mul(self, rhs: &Mat4) -> Self::Output {
        let a = self.vals;
        let b = rhs.vals;

        #[rustfmt::skip]
        let vals = [
            a[0] * b[0] + a[1] * b[4] + a[2] * b[8]  + a[3] * b[12],
            a[0] * b[1] + a[1] * b[5] + a[2] * b[9]  + a[3] * b[13],
            a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14],
            a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15],

            a[4] * b[0] + a[5] * b[4] + a[6] * b[8]  + a[7] * b[12],
            a[4] * b[1] + a[5] * b[5] + a[6] * b[9]  + a[7] * b[13],
            a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14],
            a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15],

            a[8] * b[0] + a[9] * b[4] + a[10] * b[8]  + a[11] * b[12],
            a[8] * b[1] + a[9] * b[5] + a[10] * b[9]  + a[11] * b[13],
            a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14],
            a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15],

            a[12] * b[0] + a[13] * b[4] + a[14] * b[8]  + a[15] * b[12],
            a[12] * b[1] + a[13] * b[5] + a[14] * b[9]  + a[15] * b[13],
            a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14],
            a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15],
        ];

        Mat4 { vals }
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_look_at() {
        let pos = Point3::new(1., 1., 1.);
        let at = Point3::new(0., 1., 1.);
        let up = Vec3::new(0., 1., 0.);

        let mat = Mat4::look_at(pos, at, up);

        #[rustfmt::skip]
        let expected = Mat4 {
            vals: [
                0., 0., 1., 1.,
                0., 1., 0., 1.,
                -1., 0., 0., 1.,
                0., 0. ,0., 1.,
            ],
        };

        assert_eq!(mat, expected);
    }

    #[test]
    fn matrix_multiplication() {
        #[rustfmt::skip]
        let lhs = Mat4 {
            vals: [
                5.,  7., 9., 10.,
                2.,  3., 3.,  8.,
                8., 10., 2.,  3.,
                3.,  3., 4.,  8.,
            ],
        };

        #[rustfmt::skip]
        let rhs = Mat4 {
            vals: [
                 3., 10., 12., 18.,
                12.,  1.,  4.,  9.,
                 9., 10., 12.,  2.,
                 3., 12.,  4., 10.,
            ],
        };

        #[rustfmt::skip]
        let expected = Mat4 {
            vals: [
                210., 267., 236., 271.,
                 93., 149., 104., 149.,
                171., 146., 172., 268.,
                105., 169., 128., 169.,
            ],
        };

        assert_eq!(expected, &lhs * &rhs);
    }
}
