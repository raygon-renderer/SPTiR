use std::ops::{Add, Sub};

use super::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub fn as_vector(self) -> Vector3 {
        let Point3 { x, y, z } = self;
        Vector3 { x, y, z }
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vector3;

    #[rustfmt::skip]
    fn sub(self, rhs: Point3) -> Vector3 {
        Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;
    fn sub(mut self, rhs: Vector3) -> Point3 {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;
    fn add(mut self, rhs: Vector3) -> Point3 {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
