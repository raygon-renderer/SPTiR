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
    /// The dot product (or scalar product) of two N-dimensional vectors is defined as:
    /// ```math
    /// \mathbf{a} \cdot \mathbf{b} = \sum _{i=1}^{N}{a}_{i}{b}_{i} = {a}_{1}{b}_{1} + {a}_{2}{b}_{2} + \cdots + {a}_{n}{b}_{n}
    /// ```
    /// where $\Sigma$ denotes summation.
    ///
    /// In Euclidean space, the dot product of two vectors is equivalent to the scaled angle between them:
    /// ```math
    /// \mathbf{a} \cdot \mathbf{b} = \|\mathbf{a}\|\ \|\mathbf{b}\|\cos \theta
    /// ```
    /// where $`\|\mathbf{a}\|`$ is the [Euclidean norm](#method.norm) of the vector.
    ///
    /// As such, the dot product is often used to compute the cosine theta between two normalized vectors.
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn norm(&self) -> f32 {
        let norm_squared = self.norm_squared();

        if !norm_squared.is_normal() {
            panic!("Invalid vector norm: {}", norm_squared);
        }

        norm_squared.sqrt()
    }
}
