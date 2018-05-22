use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {

    /// Returns a new Vec4 at [0, 0, 0, 0].
    pub fn new() -> Vec4 {
        Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    /// Returns a new Vec4 using the given values for x, y, z, and w.
    pub fn new_from_values(x: &f32, y: &f32, z: &f32, w: &f32) -> Vec4 {
        Vec4 { x: *x, y: *y, z: *z, w: *w }
    }

    /// Returns a new Vec4 using the 0, 1, 2, and 3 indices of the given array,
    /// where [0] -> x, [1] -> y, [2] -> z, and [3] -> w.
    pub fn new_from_array(input: &[f32; 4]) -> Vec4 {
        Vec4 {
            x: input[0],
            y: input[1],
            z: input[2],
            w: input[3]
        }
    }

    /// Returns an array of the Vec4's x, y, z, and w values where x -> [0],
    /// y -> [1], z -> [2], and w -> [3].
    pub fn to_array(&self) -> [f32; 4] {
        [ self.x, self.y, self.z, self.w ]
    }
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;

    /// Adds one Vec4 to another Vec4 and returns a new Vec4.
    fn add(self, other_vec4: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + other_vec4.x,
            y: self.y + other_vec4.y,
            z: self.z + other_vec4.z,
            w: self.w + other_vec4.w,
        }
    }
}

impl ops::AddAssign for Vec4 {

    /// Adds one Vec4 to another Vec4 and re-assigns the first Vec4 to the new
    /// Vec4.
    fn add_assign(&mut self, other_vec4: Vec4) {
        *self = Vec4 {
            x: self.x + other_vec4.x,
            y: self.y + other_vec4.y,
            z: self.z + other_vec4.z,
            w: self.w + other_vec4.w,
        }
    }
}

impl ops::Mul<Vec4> for f32 {
    type Output = Vec4;

    /// Multiplise a scalar value by a Vec4 and returns a Vec4.
    fn mul(self, other_vec4: Vec4) -> Vec4 {
        Vec4 {
            x: self * other_vec4.x,
            y: self * other_vec4.y,
            z: self * other_vec4.z,
            w: self * other_vec4.w,
        }
    }
}

impl ops::Mul<Vec4> for Vec4 {
    type Output = f32;

    /// Returns the dot product of 2 Vec4s, which is a scalar floating point.
    fn mul(self, other_vec4: Vec4) -> f32 {
        (self.x * other_vec4.x) + (self.y * other_vec4.y)
        + (self.z * other_vec4.z) + (self.w * other_vec4.w)
    }
}

impl ops::Neg for Vec4 {
    type Output = Vec4;

    /// Negates the values of Vec4, which in turn negates the Vec4.
    fn neg(self) -> Vec4 {
        Vec4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Vec4;

    /// Subtracts one Vec4 from another Vec4 and returns a new Vec4.
    fn sub(self, other_vec4: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - other_vec4.x,
            y: self.y - other_vec4.y,
            z: self.z - other_vec4.z,
            w: self.w - other_vec4.w,
        }
    }
}

impl ops::SubAssign for Vec4 {

    /// Subtracts one Vec4 from another Vec4 and re-assigns the first Vec4 to
    /// the new Vec4.
    fn sub_assign(&mut self, other_vec4: Vec4) {
        *self = Vec4 {
            x: self.x - other_vec4.x,
            y: self.y - other_vec4.y,
            z: self.z - other_vec4.z,
            w: self.w - other_vec4.w,
        };
    }
}

#[cfg(test)]
mod tests {
    use ::Vec4;

    #[test]
    fn create_new_vec4() {
        assert_eq!(Vec4::new(), Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 });
    }

    #[test]
    fn create_new_vec4_from_values() {
        let x = 0.0;
        let y = 1.0;
        let z = 2.5;
        let w = 1.5;
        let vec = Vec4::new_from_values(&x, &y, &z, &w);
        assert_eq!(vec, Vec4 { x, y, z, w });
    }

    #[test]
    fn create_new_vec4_from_array() {
        let input_array = [ 1.0, 2.5, 5.5, 3.5 ];
        let vec = Vec4::new_from_array(&input_array);
        assert_eq!(vec, Vec4 { x: 1.0, y: 2.5, z: 5.5, w: 3.5 });
    }

    #[test]
    fn get_vec4_as_array() {
        let vec = Vec4 { x: 1.0, y: 3.5, z: 0.5, w: 2.25 };
        let array = vec.to_array();
        assert_eq!(array, [ 1.0, 3.5, 0.5, 2.25 ]);
    }

    #[test]
    fn add_assign_2_vec4s() {
        let mut v1 = Vec4 { x: 1.0, y: 0.0, z: 0.5, w: 0.25 };
        let v2 = Vec4 { x: 0.0, y: 1.0, z: 0.5, w: 0.75 };
        v1 += v2;
        assert_eq!(v1, Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 });
    }

    #[test]
    fn multiply_a_vec4_by_a_scalar() {
        let c = 2.0;
        let v1 = Vec4 { x: 1.0, y: 3.5, z: 7.0, w: -2.5 };
        assert_eq!(c * v1, Vec4 { x: 2.0, y: 7.0, z: 14.0, w: -5.0 });
    }

    #[test]
    fn get_dot_product() {
        let vec1 = Vec4 { x: 3.0, y: 2.0, z: 2.0, w: 2.5 };
        let vec2 = Vec4 { x: 3.0, y: 4.0, z: 3.5, w: -0.25 };
        assert_eq!(vec1 * vec2, 23.375);
    }

    #[test]
    fn get_a_negative_vec4() {
        let v1 = Vec4 { x: 0.0, y: 2.0, z: -1.0, w: 0.5 };
        assert_eq!(-v1, Vec4 { x: 0.0, y: -2.0, z: 1.0, w: -0.5 });
    }

    #[test]
    fn subtract_2_vec4s() {
        let v1 = Vec4 { x: 1.0, y: 2.0, z: 2.0, w: 0.5 };
        let v2 = Vec4 { x: 1.0, y: 1.0, z: 3.0, w: 1.5 };
        let v3 = v1 - v2;
        assert_eq!(v3, Vec4 { x: 0.0, y: 1.0, z: -1.0, w: -1.0 });
    }

    #[test]
    fn subtract_assign_2_vec4s() {
        let mut v1 = Vec4 { x: 1.0, y: 2.0, z: 1.0, w: 0.25 };
        let v2 = Vec4 { x: 1.0, y: 2.0, z: 1.5, w: 0.75 };
        v1 -= v2;
        assert_eq!(v1, Vec4 { x: 0.0, y: 0.0, z: -0.5, w: -0.5 });
    }
}
