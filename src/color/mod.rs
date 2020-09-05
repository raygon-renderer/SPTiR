use std::marker::PhantomData;
use std::ops::{Add, Deref, DerefMut, Div, Mul};

use crate::spectrum::xyz::XYZSpectrum;

mod matrix;

pub mod spaces;

pub use self::matrix::ColorMatrix3;

/// A two-dimensional coordinate on the xy-chromaticity diagram, with an associated luminance.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct xyY {
    pub x: f32,
    pub y: f32,
    pub luminance: f32,
}

impl xyY {
    pub const fn new(x: f32, y: f32, luminance: f32) -> xyY {
        xyY { x, y, luminance }
    }

    /**
        Converts xyY to XYZ tristimulus values

        ```math
        \begin{aligned}
            X &= \frac{x Y}{y} \\
            Y &= Y \\
            Z &= \frac{(1 - x - y) Y}{y}
        \end{aligned}
        ```
    */
    pub const fn to_xyz(self) -> XYZSpectrum {
        let xyY { x, y, luminance } = self;

        let wx = luminance * x / y;
        let wy = luminance;
        let wz = luminance * (1.0 - x - y) / y;

        XYZSpectrum::new(wx, wy, wz)
    }
}

/// Defines a working space within the human chromaticity diagram for which color operations can occur
pub trait Colorspace {
    /// Defines the coordinates in the xy-chromaticity diagram for which each RGB primary is placed
    const PRIMARIES: RGBPrimaries;
    /// Defines the standard illuminant for the color space
    const WHITEPOINT: xyY;

    /// Conversion matrix from Linear RGB to XYZ tristimulus values
    const RGB_TO_XYZ: ColorMatrix3 = Self::PRIMARIES.rgb_to_xyz(Self::WHITEPOINT);
    /// Conversion matrix from XYZ tristimulus values to Linear RGB
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

/// Helper struct for Chromatic Adaptation, converting from one white-point to another.
///
/// Since color spaces are defined not only by their RGB Primaries on the xy-chromaticity chart, but
/// also their white-point, it is non-trivial to convert RGB to RGB between color spaces.
///
/// The implementation provided here uses the Bradford method of Chromatic Adaptation,
/// which has been shown to be the most perceptually accurate.
pub struct Adaptation<FROM: Colorspace, TO: Colorspace>(PhantomData<(FROM, TO)>);

/**
    Bradford cone response domain matrix

    ```math
    \begin{bmatrix}
        +0.89510  & +0.26640 & -0.16140 \\
        -0.75020  & +1.71350 & +0.03670 \\
        +0.03890  & -0.06850 & +1.02960 \\
    \end{bmatrix}
    ```
*/
#[rustfmt::skip]
pub const FORWARD_BRADFORD_MATRIX: ColorMatrix3 = ColorMatrix3([
    0.89510,  0.26640, -0.16140,
   -0.75020,  1.71350,  0.03670,
    0.03890, -0.06850,  1.02960,
]);

/// Inverse of [`FORWARD_BRADFORD_MATRIX`](`FORWARD_BRADFORD_MATRIX`), computed automatically.
///
/// [`FORWARD_BRADFORD_MATRIX`]: crate::color::FORWARD_BRADFORD_MATRIX
pub const INVERSE_BRADFORD_MATRIX: ColorMatrix3 = FORWARD_BRADFORD_MATRIX.inverse();

/**
    Computes a linear transform to adapt one whitepoint to another, using the Bradford response domains.

    ```math
    \begin{aligned}
    \begin{bmatrix} X_{D} \\ Y_{D} \\ Z_{D} \end{bmatrix} &=
    \begin{bmatrix} M \end{bmatrix}
    \begin{bmatrix} X_{S} \\ Y_{S} \\ Z_{S} \end{bmatrix} \\
    \text{where:} \\ \;
    \begin{bmatrix} M \end{bmatrix} &= \begin{bmatrix} M_{A} \end{bmatrix}^{-1}
    \begin{bmatrix}
        \rho_{D}/\rho_{S} & 0 & 0 \\
        0 & \gamma_{D}/\gamma_{S} & 0 \\
        0 & 0 & \beta_{D} / \beta_{S}
    \end{bmatrix}
    \begin{bmatrix} M_{A} \end{bmatrix} \\
    \text{where:} \\ \;
    \begin{bmatrix} \rho_{S} \\ \gamma_{S} \\ \beta_{S} \end{bmatrix} &=
        \begin{bmatrix} M_{A} \end{bmatrix} \begin{bmatrix} X_{WS} \\ Y_{WS} \\ Z_{WS} \end{bmatrix} \\ \;
    \begin{bmatrix} \rho_{D} \\ \gamma_{D} \\ \beta_{D} \end{bmatrix} &=
        \begin{bmatrix} M_{A} \end{bmatrix} \begin{bmatrix} X_{WD} \\ Y_{WD} \\ Z_{WD} \end{bmatrix}
    \end{aligned}
    ```
*/
#[rustfmt::skip]
pub const fn compute_adaptation_matrix(swp: xyY, dwp: xyY) -> ColorMatrix3 {
    // no floating point error if same
    if swp.x == dwp.x && swp.y == dwp.y && swp.luminance == dwp.luminance {
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

/// Color consisting of Red, Green and Blue components, defined within a specific colorspace.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBColor<CS: Colorspace> {
    pub r: f32,
    pub g: f32,
    pub b: f32,

    _colorspace: PhantomData<CS>,
}

/// Adapter around `RGBColor` to add alpha transparency values
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBAColor<CS: Colorspace> {
    pub rgb: RGBColor<CS>,
    pub a: f32,
}

impl<CS: Colorspace> Deref for RGBAColor<CS> {
    type Target = RGBColor<CS>;
    fn deref(&self) -> &Self::Target {
        &self.rgb
    }
}

impl<CS: Colorspace> DerefMut for RGBAColor<CS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rgb
    }
}

impl<CS: Colorspace> RGBColor<CS> {
    #[rustfmt::skip]
    pub const fn new(r: f32, g: f32, b: f32) -> RGBColor<CS> {
        RGBColor { r, g, b, _colorspace: PhantomData }
    }

    /**
        Convert the RGB value of one colorspace to an RGB value in another colorspace,
        applying chromatic adaptation to account for different whitepoints.

        ```math
            \begin{aligned}
                XYZ\_TO\_XYZ &= \texttt{\href{fn.compute_adaptation_matrix.html}{compute\_adaptation\_matrix}
                    (
                        \href{trait.Colorspace.html#associatedconstant.WHITEPOINT}{FROM::WHITEPOINT},
                        \href{trait.Colorspace.html#associatedconstant.WHITEPOINT}{TO::WHITEPOINT}
                    )}
                \\
                RGB\_TO\_RGB &= \begin{bmatrix} \texttt{
                    \href{trait.Colorspace.html#associatedconstant.XYZ_TO_RGB}{TO::XYZ\_TO\_RGB}
                } \end{bmatrix} \begin{bmatrix} XYZ\_TO\_XYZ \end{bmatrix} \begin{bmatrix} \texttt{
                    \href{trait.Colorspace.html#associatedconstant.RGB_TO_XYZ}{TO::RGB\_TO\_XYZ}
                } \end{bmatrix}
            \end{aligned}
        ```
    */
    pub const fn convert<TO: Colorspace>(self) -> RGBColor<TO> {
        Adaptation::<CS, TO>::RGB_TO_RGB.transform_rgb(self)
    }

    pub const fn to_xyz(self) -> XYZSpectrum {
        let RGBColor { r: x, g: y, b: z, .. } = CS::RGB_TO_XYZ.transform_rgb::<CS, CS>(self);
        XYZSpectrum::new(x, y, z)
    }
}

impl<CS: Colorspace> RGBAColor<CS> {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> RGBAColor<CS> {
        RGBAColor {
            rgb: RGBColor::new(r, g, b),
            a,
        }
    }

    pub const fn convert<TO: Colorspace>(self) -> RGBAColor<TO> {
        RGBAColor {
            rgb: Adaptation::<CS, TO>::RGB_TO_RGB.transform_rgb(self.rgb),
            a: self.a,
        }
    }

    pub const fn to_xyz(self) -> XYZSpectrum {
        self.rgb.to_xyz()
    }
}

/// Defines the coordinates in the xy-chromaticity diagram for which each RGB primary is placed
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBPrimaries {
    pub r: xyY,
    pub g: xyY,
    pub b: xyY,
}

impl RGBPrimaries {
    /**
    Derives the linear transformation matrix `$\begin{bmatrix}M\end{bmatrix}$` between RGB and XYZ tristimulus values,
    using the xyY RGB primaries and the whitepoint (here denoted by `$\tiny \begin{bmatrix} X_W \\ Y_W \\ Z_W \end{bmatrix}$`)

    ```math
    \begin{aligned}
        \begin{bmatrix} X \\ Y \\ Z \end{bmatrix} &=
        \begin{bmatrix} M \end{bmatrix}
        \begin{bmatrix} R \\ G \\ B \end{bmatrix}

        \\ \text{where:} \\ \;

        M &= \begin{bmatrix}
            S_r X_r & S_g X_g & S_b X_b \\
            S_r Y_r & S_g Y_g & S_b Y_b \\
            S_r Z_r & S_g Z_g & S_b Z_b
        \end{bmatrix}
        \\ \text{where:} \\ \;
        \begin{bmatrix} S_r \\ S_g \\ S_b \end{bmatrix} &=
        P_{xyz}^{-1}
        \begin{bmatrix} X_W \\ Y_W \\ Z_W \end{bmatrix}
        \\
        P_{xyz} &= \begin{pmatrix}
            X_r & Y_r & Z_r \\
            X_g & Y_g & Z_g \\
            X_b & Y_b & Z_b
        \end{pmatrix} =
        \begin{pmatrix}
            \frac{x_r Y_r}{y_r} & Y_r & \frac{(1 - x_r - y_r) Y_r}{y_r} \\ \;
            \frac{x_g Y_g}{y_g} & Y_g & \frac{(1 - x_g - y_g) Y_g}{y_g} \\ \;
            \frac{x_b Y_b}{y_b} & Y_b & \frac{(1 - x_b - y_b) Y_b}{y_b}
        \end{pmatrix}
    \end{aligned}
    ```
    */
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

impl<CS: Colorspace> Add for RGBColor<CS> {
    type Output = RGBColor<CS>;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self
    }
}

impl<CS: Colorspace> Mul for RGBColor<CS> {
    type Output = RGBColor<CS>;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self
    }
}

impl<CS: Colorspace> Mul<f32> for RGBColor<CS> {
    type Output = RGBColor<CS>;
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self
    }
}

impl<CS: Colorspace> Div<f32> for RGBColor<CS> {
    type Output = RGBColor<CS>;
    fn div(mut self, rhs: f32) -> Self::Output {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self
    }
}
