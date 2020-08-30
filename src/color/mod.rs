use std::marker::PhantomData;

use crate::spectrum::XYZSpectrum;

mod matrix;

pub mod spaces;

pub use self::matrix::ColorMatrix3;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct xyY {
    pub x: f32,
    pub y: f32,
    pub y_prime: f32,
}

impl xyY {
    pub const fn new(x: f32, y: f32, y_prime: f32) -> xyY {
        xyY { x, y, y_prime }
    }

    pub const fn to_xyz(self) -> XYZSpectrum {
        let xyY { x, y, y_prime } = self;

        let wx = y_prime * x / y;
        let wy = y_prime;
        let wz = y_prime * (1.0 - x - y) / y;

        XYZSpectrum::new(wx as f32, wy as f32, wz as f32)
    }
}

pub trait Colorspace {
    const PRIMARIES: RGBPrimaries;
    const WHITEPOINT: xyY;

    const RGB_TO_XYZ: ColorMatrix3 = Self::PRIMARIES.rgb_to_xyz(Self::WHITEPOINT);
    const XYZ_TO_RGB: ColorMatrix3 = Self::RGB_TO_XYZ.inverse();
}

pub mod whitepoints {
    use super::xyY;

    /// Noon Daylight: Television, sRGB color space
    ///
    /// The D series of illuminants are constructed to represent natural daylight.
    ///
    /// D65 has a CCT of 6504K
    pub const D65: xyY = xyY::new(0.31271, 0.32902, 1.0);

    /// D60 has a CCT of approximately 6000K
    pub const D60: xyY = xyY::new(0.32163, 0.33774, 1.0);

    /// D60-like whitepoint designed for future-compatibility of the ACES color space
    pub const ACES: xyY = xyY::new(0.32168, 0.33767, 1.0);
}

pub struct Adaptation<FROM: Colorspace, TO: Colorspace>(PhantomData<(FROM, TO)>);

#[rustfmt::skip]
pub const FORWARD_BRADFORD_MATRIX: ColorMatrix3 = ColorMatrix3([
    0.89510,  0.26640, -0.16140,
   -0.75020,  1.71350,  0.03670,
    0.03890, -0.06850,  1.02960,
]);

pub const INVERSE_BRADFORD_MATRIX: ColorMatrix3 = FORWARD_BRADFORD_MATRIX.inverse();

#[rustfmt::skip]
pub const fn compute_adaptation_matrix(swp: xyY, dwp: xyY) -> ColorMatrix3 {
    // no floating point error if same
    if swp.x == dwp.x && swp.y == dwp.y && swp.y_prime == dwp.y_prime {
        return ColorMatrix3::IDENTITY;
    }

    let cs = FORWARD_BRADFORD_MATRIX.transform_xyz(swp.to_xyz());
    let cd = FORWARD_BRADFORD_MATRIX.transform_xyz(dwp.to_xyz());

    INVERSE_BRADFORD_MATRIX
        .multiply(ColorMatrix3([
            cd.x / cs.x, 0.0, 0.0,
            0.0, cd.y / cs.y, 0.0,
            0.0, 0.0, cd.z / cs.z,
        ]))
        .multiply(FORWARD_BRADFORD_MATRIX)
}

impl<FROM: Colorspace, TO: Colorspace> Adaptation<FROM, TO> {
    pub const XYZ_TO_XYZ: ColorMatrix3 = compute_adaptation_matrix(FROM::WHITEPOINT, TO::WHITEPOINT);
    pub const RGB_TO_RGB: ColorMatrix3 = TO::XYZ_TO_RGB.multiply(Self::XYZ_TO_XYZ).multiply(FROM::RGB_TO_XYZ);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBColor<CS: Colorspace> {
    pub r: f32,
    pub g: f32,
    pub b: f32,

    _colorspace: PhantomData<CS>,
}

impl<CS: Colorspace> RGBColor<CS> {
    #[rustfmt::skip]
    pub const fn new(r: f32, g: f32, b: f32) -> RGBColor<CS> {
        RGBColor { r, g, b, _colorspace: PhantomData }
    }

    pub const fn convert<TO: Colorspace>(self) -> RGBColor<TO> {
        Adaptation::<CS, TO>::RGB_TO_RGB.transform_rgb(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBPrimaries {
    pub r: xyY,
    pub g: xyY,
    pub b: xyY,
}

impl RGBPrimaries {
    #[rustfmt::skip]
    pub const fn rgb_to_xyz(self, wp: xyY) -> ColorMatrix3 {
        let RGBPrimaries { r, g, b } = self;

        let XYZSpectrum { x: xr, y: yr, z: zr } = r.to_xyz();
        let XYZSpectrum { x: xg, y: yg, z: zg } = g.to_xyz();
        let XYZSpectrum { x: xb, y: yb, z: zb } = b.to_xyz();

        let rgb_matrix = ColorMatrix3([
            xr, xg, xb,
            yr, yg, yb,
            zr, zg, zb,
        ]);

        let XYZSpectrum { x: sr, y: sg, z: sb } =
            rgb_matrix.inverse().transform_xyz(wp.to_xyz());

        ColorMatrix3([
            sr * xr, sg * xg, sb * xb,
            sr * yr, sg * yg, sb * yb,
            sr * zr, sg * zg, sb * zb,
        ])
    }
}
