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
}
