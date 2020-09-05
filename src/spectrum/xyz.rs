//! XYZ Tristumulus values and basis functions

#[inline]
fn gaussian(lambda: f32, alpha: f32, mu: f32, sigma1: f32, sigma2: f32) -> f32 {
    let sqrt = (lambda - mu) / if lambda < mu { sigma1 } else { sigma2 };

    alpha * (-0.5 * (sqrt * sqrt)).exp()
}

/// `$\overline{x}(\lambda)$` basis function approximation
#[rustfmt::skip]
pub fn x_bar(angstroms: f32) -> f32 {
    gaussian(angstroms,  1.056, 5998.0, 379.0, 310.0) +
    gaussian(angstroms,  0.362, 4420.0, 160.0, 267.0) +
    gaussian(angstroms, -0.065, 5011.0, 204.0, 262.0)
}

/// `$\overline{y}(\lambda)$` basis function approximation
#[rustfmt::skip]
pub fn y_bar(angstroms: f32) -> f32 {
    gaussian(angstroms,  0.821, 5688.0, 469.0, 405.0) +
    gaussian(angstroms,  0.286, 5309.0, 163.0, 311.0)
}

/// `$\overline{z}(\lambda)$` basis function approximation
#[rustfmt::skip]
pub fn z_bar(angstroms: f32) -> f32 {
    gaussian(angstroms,  1.217, 4370.0, 118.0, 360.0) +
    gaussian(angstroms,  0.681, 4590.0, 260.0, 138.0)
}

use crate::color::{Colorspace, RGBColor};

/// XYZ Tristimulus color values
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XYZSpectrum {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl XYZSpectrum {
    pub const ZERO: XYZSpectrum = XYZSpectrum::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> XYZSpectrum {
        XYZSpectrum { x, y, z }
    }

    /// Converts XYZ tristimulus values to RGB within a certain colorspace.
    ///
    /// NOTE: This alone does no chromatic adaptation, and conversion from rgb to rgb of different colorspaces
    /// should be handled with [`RGBColor::convert`](`RGBColor::convert`)
    ///
    /// [`RGBColor::convert`]: crate::color::RGBColor::convert
    pub fn to_rgb<CS: Colorspace>(self) -> RGBColor<CS> {
        let XYZSpectrum { x: r, y: g, z: b } = CS::XYZ_TO_RGB.transform_xyz(self);
        RGBColor::new(r, g, b)
    }

    /**
        Calculates the tristimulus response values for the given wavelength in nanometers using
        a simple gaussian approximation.

        ```math
        g(x;\alpha ,\mu ,\sigma _{1},\sigma _{2})=\alpha \exp \left({\frac {(x-\mu )^{2}}{-2\sigma ^{2}}}\right),
        \\ {\text{ where }}\sigma ={\begin{cases}\sigma _{1},&x<\mu ,\\\sigma _{2},&x\geq \mu .\end{cases}}
        ```
        then, the response values can be calculated as:
        ```math
        \begin{aligned}
            {\overline {x}}(\lambda )&=g(\lambda ;1.056,5998,379,310)+g(\lambda ;0.362,4420,160,267)+g(\lambda ;-0.065,5011,204,262), \\
            {\overline {y}}(\lambda )&=g(\lambda ;0.821,5688,469,405)+g(\lambda ;0.286,5309,163,311),\\
            {\overline {z}}(\lambda )&=g(\lambda ;1.217,4370,118,360)+g(\lambda ;0.681,4590,260,138).
        \end{aligned}
        ```
        where `$\lambda$` is measured in Angstroms. However, the conversion from nanometers to angstroms is handled automatically here.
    */
    #[rustfmt::skip]
    pub fn from_wavelength(lambda: f32) -> XYZSpectrum {
        let angstroms = lambda * 10.0;

        XYZSpectrum::new(x_bar(angstroms), y_bar(angstroms), z_bar(angstroms))
    }
}

use std::ops::Add;

impl Add for XYZSpectrum {
    type Output = XYZSpectrum;
    fn add(mut self, rhs: XYZSpectrum) -> XYZSpectrum {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
