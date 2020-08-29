/// A 4x4 matrix
///
/// ```math
/// \begin{bmatrix}
///     m_{00} & m_{01} & m_{02} & m_{03} \\
///     m_{10} & m_{11} & m_{12} & m_{13} \\
///     m_{20} & m_{21} & m_{22} & m_{23} \\
///     m_{30} & m_{31} & m_{32} & m_{33} \\
/// \end{bmatrix}
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}

impl Matrix4 {
    /// The Identity matrix
    /// ```math
    /// \begin{bmatrix}
    ///     1 & 0 & 0 & 0 \\
    ///     0 & 1 & 0 & 0 \\
    ///     0 & 0 & 1 & 0 \\
    ///     0 & 0 & 0 & 1 \\
    /// \end{bmatrix}
    /// ```
    #[rustfmt::skip]
    pub const IDENTITY: Matrix4 = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    #[rustfmt::skip]
    pub const fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Matrix4 {
        Matrix4 {
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,
        }
    }
}
