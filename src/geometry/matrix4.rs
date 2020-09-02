use std::ops::{Add, Mul, Sub};

use super::{Point3, Vector3};

/**
    A 4x4 matrix

    ```math
    \begin{bmatrix}
        m_{00} & m_{01} & m_{02} & m_{03} \\
        m_{10} & m_{11} & m_{12} & m_{13} \\
        m_{20} & m_{21} & m_{22} & m_{23} \\
        m_{30} & m_{31} & m_{32} & m_{33} \\
    \end{bmatrix}
    ```
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4(pub [f32; 16]);

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4::IDENTITY
    }
}

impl Matrix4 {
    /**
        The Identity matrix

        ```math
        \begin{bmatrix}
            1 & 0 & 0 & 0 \\
            0 & 1 & 0 & 0 \\
            0 & 0 & 1 & 0 \\
            0 & 0 & 0 & 1 \\
        \end{bmatrix}
        ```
    */
    #[rustfmt::skip]
    pub const IDENTITY: Matrix4 = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    /// Matrix initialized to all zeroes.
    pub const ZERO: Matrix4 = Matrix4([0.0; 16]);

    #[rustfmt::skip]
    pub const fn new(
        m00: f32, m01: f32, m02: f32, m03: f32,
        m10: f32, m11: f32, m12: f32, m13: f32,
        m20: f32, m21: f32, m22: f32, m23: f32,
        m30: f32, m31: f32, m32: f32, m33: f32,
    ) -> Matrix4 {
        Matrix4([
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,
        ])
    }

    /**
    Compute `$\begin{bmatrix}M\end{bmatrix}^T$`:

    ```math
    \begin{bmatrix}
        m_{00} & m_{01} & m_{02} & m_{03} \\
        m_{10} & m_{11} & m_{12} & m_{13} \\
        m_{20} & m_{21} & m_{22} & m_{23} \\
        m_{30} & m_{31} & m_{32} & m_{33} \\
    \end{bmatrix}
    ```
    */
    #[rustfmt::skip]
    pub const fn transpose(self) -> Matrix4 {
        let Matrix4([
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,
        ]) = self;

        Matrix4([
            m00, m10, m20, m30,
            m01, m11, m21, m31,
            m02, m12, m22, m32,
            m03, m13, m23, m33,
        ])
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Matrix4 {
        let mut res = Matrix4::ZERO;

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    res.0[i * 4 + j] += self.0[i * 4 + k] * rhs.0[k * 4 + j];
                }
            }
        }

        res
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;
    fn mul(mut self, rhs: f32) -> Matrix4 {
        for x in &mut self.0 {
            *x *= rhs;
        }
        self
    }
}

impl Add for Matrix4 {
    type Output = Matrix4;
    fn add(mut self, rhs: Self) -> Matrix4 {
        for (a, b) in self.0.iter_mut().zip(&rhs.0) {
            *a += *b;
        }
        self
    }
}

impl Sub for Matrix4 {
    type Output = Matrix4;
    fn sub(mut self, rhs: Self) -> Matrix4 {
        for (a, b) in self.0.iter_mut().zip(&rhs.0) {
            *a -= *b;
        }
        self
    }
}

impl Mul<Point3> for Matrix4 {
    type Output = Point3;

    #[rustfmt::skip]
    fn mul(self, p: Point3) -> Self::Output {
        let Matrix4([
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,
        ]) = self;

        let x = m00 * p.x + m01 * p.y + m02 * p.z + m03;
        let y = m10 * p.x + m11 * p.y + m12 * p.z + m13;
        let z = m20 * p.x + m21 * p.y + m22 * p.z + m23;
        let w = m30 * p.x + m31 * p.y + m32 * p.z + m33;

        Point3::new(x / w, y / w, z / w) // normalize
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    #[rustfmt::skip]
    fn mul(self, v: Vector3) -> Self::Output {
        let Matrix4([
            m00, m01, m02, _,
            m10, m11, m12, _,
            m20, m21, m22, ..
        ]) = self;

        let x = m00 * v.x + m01 * v.y + m02 * v.z;
        let y = m10 * v.x + m11 * v.y + m12 * v.z;
        let z = m20 * v.x + m21 * v.y + m22 * v.z;

        Vector3::new(x, y, z)
    }
}
