use crate::spectrum::XYZSpectrum;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorMatrix3(pub [f32; 9]);

#[rustfmt::skip]
const fn const_abs(x: f32) -> f32 {
    if x < 0.0 { -x } else { x }
}

const fn const_is_normal(x: f32) -> bool {
    x == x && x != 0.0 && const_abs(x) < f32::INFINITY
}

impl ColorMatrix3 {
    #[rustfmt::skip]
    pub const fn const_inverse(self) -> ColorMatrix3 {
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
    pub const fn const_mul(self, b: ColorMatrix3) -> ColorMatrix3 {
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
    pub const fn const_transform(self, p: XYZSpectrum) -> XYZSpectrum {
        let ColorMatrix3([
            m00, m01, m02,
            m10, m11, m12,
            m20, m21, m22,
        ]) = self;

        let x = m00 * p.x + m01 * p.y + m02 * p.z;
        let y = m10 * p.x + m11 * p.y + m12 * p.z;
        let z = m20 * p.x + m21 * p.y + m22 * p.z;

        XYZSpectrum {x, y, z}
    }

    #[rustfmt::skip]
    pub const fn const_transpose(self) -> ColorMatrix3 {
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
