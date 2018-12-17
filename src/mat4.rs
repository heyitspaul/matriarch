//! Implementation of a 4x4 Matrix and its associated functions and methods.

use std::ops;

use super::Vec4;

/// A 4x4 Matrix with elements arraged in row-major order.
///
/// A Mat4 is laid out as follows:
///
/// ```plaintext
///     [ a  b  c  d ]
/// A = [ e  f  g  h ]
///     [ i  j  k  l ]
///     [ m  n  o  p ]
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mat4 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub l: f32,
    pub m: f32,
    pub n: f32,
    pub o: f32,
    pub p: f32,
}

impl Mat4 {
    pub fn new() -> Mat4 {
        Mat4 {
            a: 0.0, b: 0.0, c: 0.0, d: 0.0,
            e: 0.0, f: 0.0, g: 0.0, h: 0.0,
            i: 0.0, j: 0.0, k: 0.0, l: 0.0,
            m: 0.0, n: 0.0, o: 0.0, p: 0.0
        }
    }

    pub fn identity() -> Mat4 {
        Mat4 {
            a: 1.0, b: 0.0, c: 0.0, d: 0.0,
            e: 0.0, f: 1.0, g: 0.0, h: 0.0,
            i: 0.0, j: 0.0, k: 1.0, l: 0.0,
            m: 0.0, n: 0.0, o: 0.0, p: 1.0
        }
    }

    pub fn new_from_array(input: &[f32; 16]) -> Mat4 {
        Mat4 {
            a: input[0],  b: input[1],  c: input[2],  d: input[3],
            e: input[4],  f: input[5],  g: input[6],  h: input[7],
            i: input[8],  j: input[9],  k: input[10], l: input[11],
            m: input[12], n: input[13], o: input[14], p: input[15]
        }
    }

    pub fn new_from_col_array(input: &[f32; 16]) -> Mat4 {
        Mat4 {
            a: input[0], b: input[4], c: input[8],  d: input[12],
            e: input[1], f: input[5], g: input[9],  h: input[13],
            i: input[2], j: input[6], k: input[10], l: input[14],
            m: input[3], n: input[7], o: input[11], p: input[15]
        }
    }

    pub fn to_array(&self) -> [f32; 16] {
        [
            self.a, self.b, self.c, self.d,
            self.e, self.f, self.g, self.h,
            self.i, self.j, self.k, self.l,
            self.m, self.n, self.o, self.p
        ]
    }

    pub fn to_col_array(&self) -> [f32; 16] {
        [
            self.a, self.e, self.i, self.m,
            self.b, self.f, self.j, self.n,
            self.c, self.g, self.k, self.o,
            self.d, self.h, self.l, self.p
        ]
    }

    pub fn to_vec4_array(&self) -> [Vec4; 4] {
        [
            Vec4 { x: self.a, y: self.e, z: self.i, w: self.m },
            Vec4 { x: self.b, y: self.f, z: self.j, w: self.n },
            Vec4 { x: self.c, y: self.g, z: self.k, w: self.o },
            Vec4 { x: self.d, y: self.h, z: self.l, w: self.p },
        ]
    }

    pub fn determinant(&self) -> f32 {
    // This optimization, as shown in the `determinant_optimizations` 
    // benchmark, speeds up the calculation of the determinant by factoring out
    // a, b, c, and d, and then factoring out m, n, o, and p, reducing the total
    // number of multiplications needed from 72 to 40, which affords us a 57%
    // speed increase over the original code.
    //
    // Technically this could be optimized further by grouping the duplicate
    // multiplications, but it turns out the Rust compiler is already doing
    // that optimization for us.
        (self.a * (
            (self.p * ( (self.f * self.k) - (self.g * self.j) ) )
            + (self.o * ( - (self.f * self.l) + (self.h * self.j) ) )
            + (self.n * ( (self.g * self.l) - (self.h * self.k) ) )
        ))
        
        + (self.b * (
            (self.p * ( - (self.e * self.k) + (self.g * self.i) ) )
            + (self.o * ( (self.e * self.l) - (self.h * self.i) ) )
            + (self.m * ( - (self.g * self.l) + (self.h * self.k) ) )
        ))
        
        + (self.c * (
            (self.p * ( (self.e * self.j) - (self.f * self.i) ) )
            + (self.n * ( - (self.e * self.l) + (self.h * self.i) ) )
            + (self.m * ( (self.f * self.l) - (self.h * self.j) ) )
        ))
        
        + (self.d * (
            (self.o * ( - (self.e * self.j) + (self.f * self.i) ) )
            + (self.n * ( (self.e * self.k) - (self.g * self.i) ) )
            + (self.m * ( - (self.f * self.k) + (self.g * self.j) ) )
        ))
    }

    pub fn transpose(&self) -> Mat4 {
        Mat4 {
            a: self.a, b: self.e, c: self.i, d: self.m,
            e: self.b, f: self.f, g: self.j, h: self.n,
            i: self.c, j: self.g, k: self.k, l: self.o,
            m: self.d, n: self.h, o: self.l, p: self.p
        }
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    /// Multiplies two Mat4s together, returning a new Mat4.
    ///
    /// Keep in mind that matrix multiplication is not commutative, such that
    /// `A*B != B*A` for *most* matrices (the main exception being the Identity
    /// matrix)
    ///
    /// Example:
    /// ```
    /// # use matriarch::Mat4;
    /// # let mat4 = Mat4::new();
    /// # let some_other_mat4 = Mat4::new();
    /// let some_mat4 = mat4 * some_other_mat4;
    /// ```
    fn mul(self, mat4: Mat4) -> Mat4 {
        Mat4 {
            a: (self.a * mat4.a) + (self.b * mat4.e) + (self.c * mat4.i) + (self.d * mat4.m),
            b: (self.a * mat4.b) + (self.b * mat4.f) + (self.c * mat4.j) + (self.d * mat4.n),
            c: (self.a * mat4.c) + (self.b * mat4.g) + (self.c * mat4.k) + (self.d * mat4.o),
            d: (self.a * mat4.d) + (self.b * mat4.h) + (self.c * mat4.l) + (self.d * mat4.p),
            e: (self.e * mat4.a) + (self.f * mat4.e) + (self.g * mat4.i) + (self.h * mat4.m),
            f: (self.e * mat4.b) + (self.f * mat4.f) + (self.g * mat4.j) + (self.h * mat4.n),
            g: (self.e * mat4.c) + (self.f * mat4.g) + (self.g * mat4.k) + (self.h * mat4.o),
            h: (self.e * mat4.d) + (self.f * mat4.h) + (self.g * mat4.l) + (self.h * mat4.p),
            i: (self.i * mat4.a) + (self.j * mat4.e) + (self.k * mat4.i) + (self.l * mat4.m),
            j: (self.i * mat4.b) + (self.j * mat4.f) + (self.k * mat4.j) + (self.l * mat4.n),
            k: (self.i * mat4.c) + (self.j * mat4.g) + (self.k * mat4.k) + (self.l * mat4.o),
            l: (self.i * mat4.d) + (self.j * mat4.h) + (self.k * mat4.l) + (self.l * mat4.p),
            m: (self.m * mat4.a) + (self.n * mat4.e) + (self.o * mat4.i) + (self.p * mat4.m),
            n: (self.m * mat4.b) + (self.n * mat4.f) + (self.o * mat4.j) + (self.p * mat4.n),
            o: (self.m * mat4.c) + (self.n * mat4.g) + (self.o * mat4.k) + (self.p * mat4.o),
            p: (self.m * mat4.d) + (self.n * mat4.h) + (self.o * mat4.l) + (self.p * mat4.p),
        }
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, vec4: Vec4) -> Vec4 {
        Vec4 {
            x: (self.a * vec4.x) + (self.b * vec4.y) + (self.c * vec4.z) + (self.d * vec4.w),
            y: (self.e * vec4.x) + (self.f * vec4.y) + (self.g * vec4.z) + (self.h * vec4.w),
            z: (self.i * vec4.x) + (self.j * vec4.y) + (self.k * vec4.z) + (self.l * vec4.w),
            w: (self.m * vec4.x) + (self.n * vec4.y) + (self.o * vec4.z) + (self.p * vec4.w),
        }
    }
}

impl ops::Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, mat4: Mat4) -> Mat4 {
        Mat4 {
            a: self * mat4.a,
            b: self * mat4.b,
            c: self * mat4.c,
            d: self * mat4.d,
            e: self * mat4.e,
            f: self * mat4.f,
            g: self * mat4.g,
            h: self * mat4.h,
            i: self * mat4.i,
            j: self * mat4.j,
            k: self * mat4.k,
            l: self * mat4.l,
            m: self * mat4.m,
            n: self * mat4.n,
            o: self * mat4.o,
            p: self * mat4.p,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Mat4, Vec4};

    #[test]
    fn create_new_mat4() {
        assert_eq!(Mat4::new(),
            Mat4{
                a: 0.0, b: 0.0, c: 0.0, d: 0.0,
                e: 0.0, f: 0.0, g: 0.0, h: 0.0,
                i: 0.0, j: 0.0, k: 0.0, l: 0.0,
                m: 0.0, n: 0.0, o: 0.0, p: 0.0
            });
    }

    #[test]
    fn create_new_mat4_identity() {
        assert_eq!(Mat4::identity(),
            Mat4{
                a: 1.0, b: 0.0, c: 0.0, d: 0.0,
                e: 0.0, f: 1.0, g: 0.0, h: 0.0,
                i: 0.0, j: 0.0, k: 1.0, l: 0.0,
                m: 0.0, n: 0.0, o: 0.0, p: 1.0
            });
    }

    #[test]
    fn create_new_mat4_from_array() {
        let array = [ 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0 ];
        assert_eq!(
            Mat4::new_from_array(&array),
            Mat4 {
                a:  1.0, b:  2.0, c:  3.0, d:  4.0,
                e:  5.0, f:  6.0, g:  7.0, h:  8.0,
                i:  9.0, j: 10.0, k: 11.0, l: 12.0,
                m: 13.0, n: 14.0, o: 15.0, p: 16.0
            });
    }

    #[test]
    fn create_new_mat4_from_col_array() {
        let array = [ 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0 ];
        assert_eq!(
            Mat4::new_from_col_array(&array),
            Mat4 {
                a: 1.0, b: 5.0, c:  9.0, d: 13.0,
                e: 2.0, f: 6.0, g: 10.0, h: 14.0,
                i: 3.0, j: 7.0, k: 11.0, l: 15.0,
                m: 4.0, n: 8.0, o: 12.0, p: 16.0
            });
    }

    #[test]
    fn mat4_to_array() {
        let mat4 = Mat4 {
            a:  1.0, b:  2.0, c:  3.0, d:  4.0,
            e:  5.0, f:  6.0, g:  7.0, h:  8.0,
            i:  9.0, j: 10.0, k: 11.0, l: 12.0,
            m: 13.0, n: 14.0, o: 15.0, p: 16.0
        };
        assert_eq!(mat4.to_array(), [ 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0 ]);
    }

    #[test]
    fn mat4_to_col_array() {
        let mat4 = Mat4 {
            a:  1.0, b:  2.0, c:  3.0, d:  4.0,
            e:  5.0, f:  6.0, g:  7.0, h:  8.0,
            i:  9.0, j: 10.0, k: 11.0, l: 12.0,
            m: 13.0, n: 14.0, o: 15.0, p: 16.0
        };
        assert_eq!(mat4.to_col_array(), [ 1.0, 5.0, 9.0, 13.0, 2.0, 6.0, 10.0, 14.0, 3.0, 7.0, 11.0, 15.0, 4.0, 8.0, 12.0, 16.0 ]);
    }

    #[test]
    fn mat4_to_vec4_array() {
        let mat4 = Mat4 {
            a:  1.0, b:  2.0, c:  3.0, d:  4.0,
            e:  5.0, f:  6.0, g:  7.0, h:  8.0,
            i:  9.0, j: 10.0, k: 11.0, l: 12.0,
            m: 13.0, n: 14.0, o: 15.0, p: 16.0
        };
        let array = [
            Vec4 { x: 1.0, y: 5.0, z: 9.0, w: 13.0 },
            Vec4 { x: 2.0, y: 6.0, z: 10.0, w: 14.0 },
            Vec4 { x: 3.0, y: 7.0, z: 11.0, w: 15.0 },
            Vec4 { x: 4.0, y: 8.0, z: 12.0, w: 16.0 }
        ];
        assert_eq!(mat4.to_vec4_array(), array);
    }

    #[test]
    fn get_determinant_of_mat4() {
        let array = [ 2.0, 3.0, 5.0, -1.0, 7.0, 1.0, 2.0, 0.0, 5.0, 1.0, 0.0, 2.5, 8.0, 1.0, 1.0, 3.25 ];
        let mat4 = Mat4::new_from_array(&array);
        assert_eq!(mat4.determinant(), 63.0);
    }

    #[test]
    fn get_determinant_equal_to_zero() {
        let array = [1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 6.0, 8.0, 1.0, 2.25, 4.0, 7.0, 12.0, 2.0, 4.0, -3.0 ];
        let mat4 = Mat4::new_from_array(&array);
        assert_eq!(mat4.determinant(), 0.0);
    }

    #[test]
    fn multiply_by_identity() {
        let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
        let mat4 = Mat4::new_from_array(&array);
        let iden = Mat4::identity();
        assert_eq!(mat4 * iden, mat4);
    }

    #[test]
    fn mat4_multiplication() {
        let array = [ -8.0, -7.0, -6.0, -5.0, -4.0, -3.0, -2.0, -1.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0 ];
        let mat4 = Mat4::new_from_array(&array);
        let other_array = [ 1.0, 10.0, 8.0, 0.0, -6.0, -1.0, -5.0, -4.0, 14.0, 15.0, 14.0, 2.0, -8.0, -4.0, 7.0, 2.0 ];
        let other_mat4 = Mat4::new_from_array(&other_array);
        assert_eq!(
            mat4 * other_mat4,
            Mat4 {
                a: -10.0, b: -143.0, c: -148.0, d: 6.0,
                e:  -6.0, f:  -63.0, g:  -52.0, h: 6.0,
                i:  -1.0, j:   37.0, k:   68.0, l: 6.0,
                m:   3.0, n:  117.0, o:  164.0, p: 6.0
            });
    }

    #[test]
    fn mat4_scalar_multiplication() {
        let scalar = 2.0;
        let array = [ -8.0, -7.0, -6.0, -5.0, -4.0, -3.0, -2.0, -1.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0 ];
        let mat4 = Mat4::new_from_array(&array);
        assert_eq!(
            scalar * mat4,
            Mat4 {
                a: -16.0, b: -14.0, c: -12.0, d: -10.0,
                e:  -8.0, f:  -6.0, g:  -4.0, h:  -2.0,
                i:   2.0, j:   4.0, k:   6.0, l:   8.0,
                m:  10.0, n:  12.0, o:  14.0, p:  16.0
            });
    }

    #[test]
    fn multiply_mat4_by_vec4() {
        let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
        let mat4 = Mat4::new_from_array(&array);
        let vec4 = Vec4::new_from_values(&8.0, &5.0, &0.0, &5.0);
        assert_eq!(
            mat4 * vec4,
            Vec4::new_from_values(&64.5, &150.0, &58.0, &81.0)
        );
    }
}
