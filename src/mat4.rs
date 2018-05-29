use std::ops;

use ::Vec4;

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
        (self.a * self.f * self.k * self.p) - (self.a * self.f * self.l * self.o) - 
        (self.a * self.g * self.j * self.p) + (self.a * self.g * self.l * self.n) +
        (self.a * self.h * self.j * self.o) - (self.a * self.h * self.k * self.n) -
        (self.b * self.e * self.k * self.p) + (self.b * self.e * self.l * self.o) +
        (self.b * self.g * self.i * self.p) - (self.b * self.g * self.l * self.m) -
        (self.b * self.h * self.i * self.o) + (self.b * self.h * self.k * self.m) +
        (self.c * self.e * self.j * self.p) - (self.c * self.e * self.l * self.n) -
        (self.c * self.f * self.i * self.p) + (self.c * self.f * self.l * self.m) +
        (self.c * self.h * self.i * self.n) - (self.c * self.h * self.j * self.m) -
        (self.d * self.e * self.j * self.o) + (self.d * self.e * self.k * self.n) +
        (self.d * self.f * self.i * self.o) - (self.d * self.f * self.k * self.m) -
        (self.d * self.g * self.i * self.n) + (self.d * self.g * self.j * self.m)
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
            p: self * mat4.p
        }
    }
}

#[cfg(test)]
mod tests {
    use ::Vec4;
    use ::Mat4;

    #[test]
    fn get_determinant_of_mat4() {
        let array = [ 2.0, 3.0, 5.0, -1.0, 7.0, 1.0, 2.0, 0.0, 5.0, 1.0, 0.0, 2.5, 8.0, 1.0, 1.0, 3.25 ];
        let mat4 = Mat4::new_from_array(&array);
        assert_eq!(mat4.determinant(), 63.0);
    }

    #[test]
    fn multiply_by_identity() {
        let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
        let mat4 = Mat4::new_from_array(&array);
        let iden = Mat4::identity();
        assert_eq!(mat4 * iden, mat4);
    }

    #[test]
    fn multiply_mat4_by_vec4() {
        let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
        let mat4 = Mat4::new_from_array(&array);
        let vec4 = Vec4::new_from_values(&8.0, &5.0, &0.0, &5.0);
        assert_eq!(mat4 * vec4, Vec4::new_from_values(&64.5, &150.0, &58.0, &81.0));
    }
}
