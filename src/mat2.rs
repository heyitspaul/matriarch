use std::ops;

use ::Vec2;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mat2 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl Mat2 {
    
    /// Returns a new array with all elements set to 0.0
    pub fn new() -> Mat2 {
        Mat2 {
            a: 0.0, b: 0.0,
            c: 0.0, d: 0.0
        }
    }

    /// Returns an identity matrix.
    pub fn identity() -> Mat2 {
        Mat2 {
            a: 1.0, b: 0.0,
            c: 0.0, d: 1.0
        }
    }

    pub fn new_from_values(a: &f32, b: &f32, c: &f32, d: &f32) -> Mat2 {
        Mat2 {
            a: *a, b: *b,
            c: *c, d: *d
        }
    }

    pub fn new_from_array(input: &[f32; 4]) -> Mat2 {
        Mat2 {
            a: input[0], b: input[1],
            c: input[2], d: input[3]
        }
    }

    /// Creates a Mat2 from a column-major ordered array.
    pub fn new_from_col_array(input: &[f32; 4]) -> Mat2 {
        Mat2 {
            a: input[0], b: input[2],
            c: input[1], d: input[3]
        }
    }

    pub fn to_array(&self) -> [f32; 4] {
        [ self.a, self.b, self.c, self.d ]
    }

    /// Returns an array of the Mat2 elements in column-major order.
    pub fn to_col_array(&self) -> [f32; 4] {
        [ self.a, self.c, self.b, self.d ]
    }

    /// Returns the matrix as an array of Vec2 columns
    pub fn to_vec2_array(&self) -> [Vec2; 2] {
        [
            Vec2 { x: self.a, y: self.c },
            Vec2 { x: self.b, y: self.d }
        ]
    }
}

impl ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;

    /// Multiplies two Mat2s together, returning a new Mat2.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Mat2;
    /// # let mat2 = Mat2::new();
    /// # let identity = Mat2::identity();
    /// let some_mat2 = mat2 * identity;
    /// ```
    fn mul(self, other_mat2: Mat2) -> Mat2 {
        Mat2 {
            a: (self.a * other_mat2.a) + (self.b * other_mat2.c),
            b: (self.a * other_mat2.b) + (self.b * other_mat2.d),
            c: (self.c * other_mat2.a) + (self.d * other_mat2.c),
            d: (self.c * other_mat2.b) + (self.d * other_mat2.d)
        }
    }
}

impl ops::Mul<Mat2> for f32 {
    type Output = Mat2;

    /// Multiplies a Mat2 and a scalar together and returns the Mat2.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Mat2;
    /// let scalar = 2.0;
    /// let some_mat2 = Mat2::identity();
    /// let result = scalar * some_mat2;
    /// ```
    fn mul(self, mat2: Mat2) -> Mat2 {
        Mat2 {
            a: self * mat2.a,
            b: self * mat2.b,
            c: self * mat2.c,
            d: self * mat2.d
        }
    }
}

