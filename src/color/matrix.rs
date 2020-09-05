use crate::color::{Colorspace, RGBColor};
use crate::spectrum::xyz::XYZSpectrum;

/// Simple 3x3 Matrix for mostly compile-time computations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorMatrix3(pub [f32; 9]);

const fn const_is_normal(x: f32) -> bool {
    x == x && x != 0.0 && x < f32::INFINITY && x > f32::NEG_INFINITY
}

impl ColorMatrix3 {
    #[rustfmt::skip]
    pub const IDENTITY: ColorMatrix3 = ColorMatrix3([
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ]);

    #[rustfmt::skip]
    pub const fn inverse(self) -> ColorMatrix3 {
        let ColorMatrix3([
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22,
        ]) = self;

        let det = m00 * (m11 * m22 - m21 * m12) -
                       m01 * (m10 * m22 - m12 * m20) +
                       m02 * (m10 * m21 - m11 * m20);

        if !const_is_normal(det) {
            panic!("Unable to compute matrix inverse")
        }

        let inv_det = 1.0 / det;

        let im00 = (m11 * m22 - m21 * m12) * inv_det;
        let im01 = (m02 * m21 - m01 * m22) * inv_det;
        let im02 = (m01 * m12 - m02 * m11) * inv_det;
        let im10 = (m12 * m20 - m10 * m22) * inv_det;
        let im11 = (m00 * m22 - m02 * m20) * inv_det;
        let im12 = (m10 * m02 - m00 * m12) * inv_det;
        let im20 = (m10 * m21 - m20 * m11) * inv_det;
        let im21 = (m20 * m01 - m00 * m21) * inv_det;
        let im22 = (m00 * m11 - m10 * m01) * inv_det;

        ColorMatrix3([
            im00, im01, im02,
            im10, im11, im12,
            im20, im21, im22,
        ])
    }

    #[rustfmt::skip]
    pub const fn multiply(self, b: ColorMatrix3) -> ColorMatrix3 {
        let mut res = [0.0; 9];

        let mut i = 0; while i < 3 {
            let mut j = 0; while j < 3 {
                let mut k = 0; while k < 3 {
                    res[i * 3 + j] += self.0[i * 3 + k] * b.0[k * 3 + j];
                    k += 1;
                } j += 1;
            } i += 1;
        }

        ColorMatrix3(res)
    }

    #[rustfmt::skip]
    const fn real_transform_3d(self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        let ColorMatrix3([
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22,
        ]) = self;

        (
            m00 * x + m01 * y + m02 * z,
            m10 * x + m11 * y + m12 * z,
            m20 * x + m21 * y + m22 * z,
        )
    }

    pub const fn transform_xyz(self, p: XYZSpectrum) -> XYZSpectrum {
        let (x, y, z) = self.real_transform_3d(p.x, p.y, p.z);
        XYZSpectrum::new(x, y, z)
    }

    /// WARNING: This method does not actually calculate the color adaptation matrix and should not be used directly.
    pub const fn transform_rgb<FROM: Colorspace, TO: Colorspace>(self, p: RGBColor<FROM>) -> RGBColor<TO> {
        let (r, g, b) = self.real_transform_3d(p.r, p.g, p.b);
        RGBColor::new(r, g, b)
    }

    #[rustfmt::skip]
    pub const fn transpose(self) -> ColorMatrix3 {
        let ColorMatrix3([
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22,
        ]) = self;

        ColorMatrix3([
            m00, m10, m20,
            m01, m11, m21,
            m02, m12, m22,
        ])
    }
}
