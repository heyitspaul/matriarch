//! Implementation of a 2x2 Matrix and its associated functions and methods.

use std::ops;

use Vec2;

/// A 2x2 Matrix with elements arranged in row-major order.
/// 
/// A Mat2 is laid out as follows:
/// 
/// ```plaintext
///     [ a  b ]
/// A = [ c  d ]
/// ```
/// 
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

    /// Creates a new Mat2 using the borrowed values.
    pub fn new_from_values(a: &f32, b: &f32, c: &f32, d: &f32) -> Mat2 {
        Mat2 {
            a: *a, b: *b,
            c: *c, d: *d
        }
    }

    /// Creates a new Mat2 from a row-major ordered array.
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

    /// Returns an array of the Mat2 elements in row-major order.
    pub fn to_array(&self) -> [f32; 4] {
        [self.a, self.b, self.c, self.d]
    }

    /// Returns an array of the Mat2 elements in column-major order.
    pub fn to_col_array(&self) -> [f32; 4] {
        [self.a, self.c, self.b, self.d]
    }

    /// Returns the matrix as an array of Vec2 columns
    pub fn to_vec2_array(&self) -> [Vec2; 2] {
        [
            Vec2 { x: self.a, y: self.c },
            Vec2 { x: self.b, y: self.d }
        ]
    }

    /// Returns the determinant of a Mat2.
    pub fn determinant(&self) -> f32 {
        (self.a * self.d) - (self.b * self.c)
    }

    /// Returns a new Mat2 that is the tranpose of the original Mat2.
    pub fn transpose(&self) -> Mat2 {
        Mat2 {
            a: self.a, b: self.c,
            c: self.b, d: self.d
        }
    }
}

impl ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;

    /// Multiplies two Mat2s together, returning a new Mat2.
    /// 
    /// Keep in mind that matrix multiplication is not commutative, such that
    /// `A*B != B*A` for *most* matrices (the main exception being the Identity
    /// matrix)
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Mat2;
    /// # let mat2 = Mat2::new();
    /// # let some_other_mat2 = Mat2::new();
    /// let some_mat2 = mat2 * some_other_mat2;
    /// ```
    fn mul(self, other_mat2: Mat2) -> Mat2 {
        Mat2 {
            a: (self.a * other_mat2.a) + (self.b * other_mat2.c),
            b: (self.a * other_mat2.b) + (self.b * other_mat2.d),
            c: (self.c * other_mat2.a) + (self.d * other_mat2.c),
            d: (self.c * other_mat2.b) + (self.d * other_mat2.d),
        }
    }
}

impl ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;

    /// Multiplies a Mat2 by a Vec2, returning a Vec2.
    fn mul(self, vec2: Vec2) -> Vec2 {
        Vec2 {
            x: (self.a * vec2.x) + (self.b * vec2.y),
            y: (self.c * vec2.x) + (self.d * vec2.y),
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
            d: self * mat2.d,
        }
    }
}

#[cfg(test)]
mod tests {
    use Mat2;
    use Vec2;

    #[test]
    fn create_new_mat2() {
        assert_eq!(Mat2::new(), Mat2 { a: 0.0, b: 0.0, c: 0.0, d: 0.0 });
    }

    #[test]
    fn create_new_mat2_identity() {
        assert_eq!(Mat2::identity(), Mat2{ a: 1.0, b: 0.0, c: 0.0, d: 1.0 });
    }

    #[test]
    fn create_new_mat2_from_values() {
        assert_eq!(Mat2::new_from_values(&1.0, &2.0, &3.0, &4.0), Mat2 { a: 1.0, b: 2.0, c: 3.0, d: 4.0 });
    }

    #[test]
    fn create_new_mat2_from_array() {
        let values = [1.0, 2.0, 3.0, 4.0];
        let mat2 = Mat2::new_from_array(&values);
        assert_eq!(mat2, Mat2{ a: 1.0, b: 2.0, c: 3.0, d: 4.0 });
    }

    #[test]
    fn create_new_mat2_from_col_array() {
        let values = [1.0, 2.0, 3.0, 4.0];
        let mat2 = Mat2::new_from_col_array(&values);
        assert_eq!(mat2, Mat2{ a: 1.0, c: 2.0, b: 3.0, d: 4.0 });
    }

    #[test]
    fn mat2_to_array() {
        let mat2 = Mat2 { a: 1.0, b: 2.0, c: 3.0, d: 4.0 };
        let array = mat2.to_array();
        assert_eq!(array, [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn mat2_to_col_array() {
        let mat2 = Mat2 { a: 1.0, b: 2.0, c: 3.0, d: 4.0 };
        let array = mat2.to_col_array();
        assert_eq!(array, [1.0, 3.0, 2.0, 4.0]);
    }

    #[test]
    fn mat2_to_vec2_array() {
        let mat2 = Mat2 { a: 1.0, b: 2.0, c: 3.0, d: 4.0 };
        let array = mat2.to_vec2_array();
        let other_array = [Vec2 { x: 1.0, y: 3.0 }, Vec2 { x: 2.0, y: 4.0 }];
        assert_eq!(array, other_array);
    }

    #[test]
    fn get_determinant_of_mat2() {
        let mat2 = Mat2 { a: 2.0, b: 3.0, c: 7.0, d: 1.0 };
        assert_eq!(mat2.determinant(), -19.0);
    }

    #[test]
    fn get_determinant_equal_to_zero() {
        let mat2 = Mat2 { a: 2.0, b: 3.0, c: 4.0, d: 6.0 };
        assert_eq!(mat2.determinant(), 0.0);
    }

    #[test]
    fn multiply_by_identity() {
        let mat2 = Mat2::new_from_values(&2.0, &3.0, &4.0, &5.0);
        let iden = Mat2::identity();
        assert_eq!(mat2 * iden, mat2);
    }

    #[test]
    fn mat2_multiplication() {
        let mat2 = Mat2::new_from_values(&1.0, &2.0, &1.0, &3.0);
        let other_mat2 = Mat2::new_from_values(&1.5, &2.25, &1.25, &2.0);
        assert_eq!(
            mat2 * other_mat2,
            Mat2::new_from_values(&4.0, &6.25, &5.25, &8.25)
        );
    }

    #[test]
    fn mat2_scalar_multiplication() {
        let scalar = 2.0;
        let mat2 = Mat2::new_from_values(&1.0, &3.0, &1.5, &2.0);
        assert_eq!(scalar * mat2, Mat2::new_from_values(&2.0, &6.0, &3.0, &4.0));
    }

    #[test]
    fn multiply_mat2_by_vec2() {
        let mat2 = Mat2::new_from_values(&1.0, &2.0, &3.0, &2.0);
        let vec2 = Vec2::new_from_values(&4.0, &5.0);
        assert_eq!(mat2 * vec2, Vec2::new_from_values(&14.0, &22.0));
    }
}
