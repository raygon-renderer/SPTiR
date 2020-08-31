use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    pub const UP: Vector3 = Vector3::new(0.0, 1.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }
}

impl Vector3 {
    /**
        The dot product (or scalar product) of two N-dimensional vectors is defined as:
        ```math
        \mathbf{a} \cdot \mathbf{b} = \sum _{i=1}^{N}{a}_{i}{b}_{i} = {a}_{1}{b}_{1} + {a}_{2}{b}_{2} + \cdots + {a}_{n}{b}_{n}
        ```

        where `$\Sigma$` denotes summation.

        In Euclidean space, the dot product of two vectors is equivalent to the scaled angle between them:
        ```math
        \mathbf{a} \cdot \mathbf{b} = \|\mathbf{a}\|\ \|\mathbf{b}\|\cos \theta
        ```
        where `$\|\mathbf{a}\|$` is the [Euclidean norm](#method.norm) of the vector.

        As such, the dot product is often used to compute the cosine theta between two normalized vectors.
    */
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /**
        The cross product (or vector product) is a binary operation on two vectors specifically in three-dimensional space,
        and is denoted as the symbol `$\times$`

        `Testing`

        ```math
        \mathbf{a} \times \mathbf{b} =\left\|\mathbf{a} \right\|\left\|\mathbf{b} \right\|\sin(\theta )\ \mathbf{n}
        ```
        where `$\theta$` is the angle between `$\mathbf{a}$` and `$\mathbf{b}$` and `$\mathbf{n}$` is a unit vector perpendicular
        to the plane containing `$\mathbf{a}$` and `$\mathbf{b}$`.

        If the vectors `$\mathbf{a}$` and `$\mathbf{b}$` are parallel, by the above formula, the cross product is a zero-vector.

        By convention, the direction of the vector `$\mathbf{n}$` is given by the right-hand rule for a right-handed coordinate system.

        For a right-handed coordinate system, the cross-product boils down to:
        ```math
        {\begin{pmatrix}
            s_{1}\\s_{2}\\s_{3}
        \end{pmatrix}} =
        {\begin{pmatrix}
            a_{2}b_{3}-a_{3}b_{2} \\
            a_{3}b_{1}-a_{1}b_{3} \\
            a_{1}b_{2}-a_{2}b_{1}
        \end{pmatrix}}
        ```
    */
    pub fn cross(&self, other: &Self) -> Vector3 {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y - other.x;

        Vector3 { x, y, z }
    }

    pub fn norm_squared(&self) -> f32 {
        self.dot(self)
    }

    /**
        Compute the Euclidean norm (`$\|\mathbf{v}\|$`) of the vector `$\mathbf{v}$`

        ```math
        \|\mathbf{v}\| = \sqrt{\mathbf{v} \cdot \mathbf{v}}
        ```
        where `$\mathbf{v} \cdot \mathbf{v}$` is the [dot product](#method.dot).

        By computing the dot product of itself, the result is `$\|\mathbf{v}\| \|\mathbf{v}\|\cos\theta$`,
        and because cosine theta between two identical vectors is 1.0, the result is the norm squared.
    */
    pub fn norm(&self) -> f32 {
        self.norm_squared().sqrt()
    }

    /**
        Normalizes the vector such that the 3D [Euclidean norm](#method.norm) is 1.0

        ```math
        \mathbf{v}_{normalized} = \frac{\mathbf{v}}{\|\mathbf{v}\|}
        ```
    */
    pub fn normalize(&self) -> Vector3 {
        let norm_squared = self.norm_squared();

        if !norm_squared.is_normal() {
            panic!("Invalid vector norm: {}", norm_squared);
        }

        *self / norm_squared.sqrt()
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(mut self, rhs: Self) -> Vector3 {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(mut self, rhs: Self) -> Vector3 {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(mut self, rhs: f32) -> Vector3 {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(mut self, rhs: f32) -> Vector3 {
        debug_assert!(rhs.is_normal());

        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;

        self
    }
}
