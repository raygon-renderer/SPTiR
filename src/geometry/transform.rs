use std::ops::{Div, Mul};

use super::Matrix4;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Transform3 {
    pub forward: Matrix4,
    pub inverse: Matrix4,
}

impl Transform3 {
    pub const fn invert(self) -> Transform3 {
        Transform3 {
            forward: self.inverse,
            inverse: self.forward,
        }
    }
}

impl Mul for Transform3 {
    type Output = Transform3;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Transform3 {
        Transform3 {
            forward: self.forward * rhs.forward,
            inverse: rhs.inverse * self.inverse,
        }
    }
}

impl Div for Transform3 {
    type Output = Transform3;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn div(self, rhs: Transform3) -> Transform3 {
        Transform3 {
            forward: rhs.forward * self.inverse,
            inverse: self.inverse * rhs.forward,
        }
    }
}
