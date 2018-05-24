use std::ops;

use ::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mat3 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub i: f32,
}

impl Mat3 {

    pub fn new() -> Mat3 {
        Mat3 {
            a: 0.0, b: 0.0, c: 0.0,
            d: 0.0, e: 0.0, f: 0.0,
            g: 0.0, h: 0.0, i: 0.0
        }
    }

    pub fn identity() -> Mat3 {
        Mat3 {
            a: 1.0, b: 0.0, c: 0.0,
            d: 0.0, e: 1.0, f: 0.0,
            g: 0.0, h: 0.0, i: 1.0
        }
    }

    pub fn new_from_values(a: &f32, b: &f32, c: &f32, d: &f32, e: &f32,
            f: &f32, g: &f32, h: &f32, i: &f32) -> Mat3 {
        Mat3 {
            a: *a, b: *b, c: *c,
            d: *d, e: *e, f: *f,
            g: *g, h: *h, i: *i
        }
    }

    pub fn new_from_array(input: &[f32; 9]) -> Mat3 {
        Mat3 {
            a: input[0], b: input[1], c: input[2],
            d: input[3], e: input[4], f: input[5],
            g: input[6], h: input[7], i: input[8]
        }
    }

    pub fn new_from_col_array(input: &[f32; 9]) -> Mat3 {
        Mat3 {
            a: input[0], b: input[3], c: input[6],
            d: input[1], e: input[4], f: input[7],
            g: input[2], h: input[5], i: input[8]
        }
    }

    pub fn to_array(&self) -> [f32; 9] {
        [ self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i]
    }

    pub fn to_col_array(&self) -> [f32; 9] {
        [ self.a, self.d, self.g, self.b, self.e, self.h, self.c, self.f, self.i]
    }

    pub fn to_vec3_array(&self) -> [Vec3; 3] {
        [
            Vec3 { x: self.a, y: self.d, z: self.g },
            Vec3 { x: self.b, y: self.e, z: self.h },
            Vec3 { x: self.c, y: self.f, z: self.i }
        ]
    }
}

impl ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, other_mat3: Mat3) -> Mat3 {
        Mat3 {
            a: (self.a * other_mat3.a) + (self.b * other_mat3.d) + (self.c * other_mat3.g),
            b: (self.a * other_mat3.b) + (self.b * other_mat3.e) + (self.c * other_mat3.h),
            c: (self.a * other_mat3.c) + (self.b * other_mat3.f) + (self.c * other_mat3.i),
            d: (self.d * other_mat3.a) + (self.e * other_mat3.d) + (self.f * other_mat3.g),
            e: (self.d * other_mat3.b) + (self.e * other_mat3.e) + (self.f * other_mat3.h),
            f: (self.d * other_mat3.c) + (self.e * other_mat3.f) + (self.f * other_mat3.i),
            g: (self.g * other_mat3.a) + (self.h * other_mat3.d) + (self.i * other_mat3.g),
            h: (self.g * other_mat3.b) + (self.h * other_mat3.e) + (self.i * other_mat3.h),
            i: (self.g * other_mat3.c) + (self.h * other_mat3.f) + (self.i * other_mat3.i)
        }
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, vec3: Vec3) -> Vec3 {
        Vec3 {
            x: (self.a * vec3.x) + (self.b * vec3.y) + (self.c * vec3.z),
            y: (self.d * vec3.x) + (self.e * vec3.y) + (self.f * vec3.z),
            z: (self.g * vec3.x) + (self.h * vec3.y) + (self.i * vec3.z)
        }
    }
}

impl ops::Mul<Mat3> for f32 {
    type Output = Mat3;

    fn mul(self, mat3: Mat3) -> Mat3 {
        Mat3 {
            a: self * mat3.a,
            b: self * mat3.b,
            c: self * mat3.c,
            d: self * mat3.d,
            e: self * mat3.e,
            f: self * mat3.f,
            g: self * mat3.g,
            h: self * mat3.h,
            i: self * mat3.i
        }
    }
}

#[cfg(test)]
mod tests {
    use ::Vec3;
    use ::Mat3;

    #[test]
    fn multiply_by_identity() {
        let array = [ 1.0, 2.5, 2.0, 9.5, 8.0, 0.0, 1.0, 1.0, 6.5 ];
        let mat3 = Mat3::new_from_array(&array);
        let iden = Mat3::identity();
        assert_eq!(mat3 * iden, mat3);
    }

    #[test]
    fn multiply_mat3_by_vec3() {
        let array = [ 1.0, 2.5, 2.0, 9.5, 8.0, 0.0, 1.0, 1.0, 6.5 ];
        let mat3 = Mat3::new_from_array(&array);
        let vec3 = Vec3::new_from_values(&2.0, &2.5, &3.5);
        assert_eq!(mat3 * vec3, Vec3::new_from_values(&15.25, &39.0, &27.25));
    }
}