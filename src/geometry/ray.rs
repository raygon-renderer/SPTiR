use super::{Point3, Vector3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub tmax: f32,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t.min(self.tmax)
    }
}
