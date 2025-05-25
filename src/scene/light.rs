use crate::math::{Color, Point3, Vector3};

pub enum Light {
    Ambient { color: Color },
    Parallel { color: Color, direction: Vector3 },
    Point { color: Color, position: Point3 },
}
