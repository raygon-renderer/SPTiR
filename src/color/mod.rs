use std::marker::PhantomData;

use crate::spectrum::XYZSpectrum;

mod matrix;

pub use self::matrix::ColorMatrix3;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct xyY {
    pub x: f32,
    pub y: f32,
    pub y_prime: f32,
}

pub trait Colorspace {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBColor<CS: Colorspace> {
    pub r: f32,
    pub g: f32,
    pub b: f32,

    _colorspace: PhantomData<CS>,
}

impl xyY {
    pub const fn to_xyz(self) -> XYZSpectrum {
        let xyY { x, y, y_prime } = self;

        let wx = y_prime * x / y;
        let wy = y_prime;
        let wz = y_prime * (1.0 - x - y) / y;

        XYZSpectrum::new(wx as f32, wy as f32, wz as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBPrimaries {
    pub r: xyY,
    pub g: xyY,
    pub b: xyY,
}

impl RGBPrimaries {
    pub const fn rgb_to_xyz(self, wp: XYZSpectrum) -> ColorMatrix3 {
        ColorMatrix3([0.0; 9])
    }
}
