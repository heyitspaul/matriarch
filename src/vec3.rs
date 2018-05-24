use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {

    /// Returns a new Vec3 at [0, 0, 0].
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// let vec3 = Vec3::new();
    /// ```
    pub fn new() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Returns a new Vec3 using the given values for x, y, and z.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// let x: f32 = 1.0;
    /// let y: f32 = 2.0;
    /// let z: f32 = 3.0;
    /// let vec3 = Vec3::new_from_values(&x, &y, &z);
    /// ```
    pub fn new_from_values(x: &f32, y: &f32, z: &f32) -> Vec3 {
        Vec3 { x: *x, y: *y, z: *z }
    }

    /// Returns a new Vec3 using the 0, 1, and 2 indices of the given array,
    /// where [0] -> x, [1] -> y, and [2] -> z.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// let input = [ 1.0, 2.0, 3.0 ];
    /// let vec3 = Vec3::new_from_array(&input);
    /// ```
    pub fn new_from_array(input: &[f32; 3]) -> Vec3 {
        Vec3 { x: input[0], y: input[1], z: input[2] }
    }

    /// Returns an array of the Vec3's x, y and z values where x -> [0],
    /// y -> [1], and z -> [2].
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let some_vec3 = Vec3::new();
    /// let array = some_vec3.to_array();
    /// ```
    pub fn to_array(&self) -> [f32; 3] {
        [ self.x, self.y, self.z ]
    }

    /// Returns the cross product of 2 Vec3s as a Vec3.
    /// 
    /// The cross product of 2 Vec3s is defined as a Vec3 that is perpendicular
    /// to both Vec3s.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let some_vec3 = Vec3::new();
    /// # let some_other_vec3 = Vec3::new();
    /// let cross_product = some_vec3.cross_product(&some_other_vec3);
    /// ```
    pub fn cross_product(&self, other_vec3: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other_vec3.z) - (self.z * other_vec3.y),
            y: (self.z * other_vec3.x) - (self.x * other_vec3.z),
            z: (self.x * other_vec3.y) - (self.y * other_vec3.x)
        }
    }

    /// Returns the length of the Vec3.
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    /// Adds one Vec3 to another Vec3 and returns a new Vec3.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let some_vec3 = Vec3::new();
    /// # let some_other_vec3 = Vec3::new();
    /// let vec3 = some_vec3 + some_other_vec3;
    /// ```
    fn add(self, other_vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other_vec3.x,
            y: self.y + other_vec3.y,
            z: self.z + other_vec3.z
        }
    }
}

impl ops::AddAssign for Vec3 {

    /// Adds one Vec3 to another Vec3 and re-assigns the first Vec3 to the new
    /// Vec3
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let mut some_vec3 = Vec3::new();
    /// # let some_other_vec3 = Vec3::new();
    /// some_vec3 += some_other_vec3;
    /// ```
    fn add_assign(&mut self, other_vec3: Vec3) {
        *self = Vec3 {
            x: self.x + other_vec3.x,
            y: self.y + other_vec3.y,
            z: self.z + other_vec3.z
        };
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    /// Multiplies a scalar value by a Vec3 and returns a Vec3.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let some_scalar = 3.0;
    /// # let some_vec3 = Vec3::new();
    /// let scaled_vec3: Vec3 = some_scalar * some_vec3;
    /// ```
    fn mul(self, other_vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self * other_vec3.x,
            y: self * other_vec3.y,
            z: self * other_vec3.z
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f32;

    /// Retuns the dot product of 2 Vec3s, which is a scalar floating point.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let some_vec3 = Vec3::new();
    /// # let some_other_vec3 = Vec3::new();
    /// let scalar: f32 = some_vec3 * some_other_vec3;
    /// ```
    fn mul(self, other_vec3: Vec3) -> f32 {
        (self.x * other_vec3.x) + (self.y * other_vec3.y) + (self.z * other_vec3.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    /// Negates the values of Vec3, which in turn negates the Vec3.
    fn neg(self) -> Vec3 {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    /// Subtracts one Vec3 from another Vec3 and returns a new Vec3.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let some_vec3 = Vec3::new();
    /// # let some_other_vec3 = Vec3::new();
    /// let vec3 = some_vec3 - some_other_vec3;
    /// ```
    fn sub(self, other_vec3: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other_vec3.x,
            y: self.y - other_vec3.y,
            z: self.z - other_vec3.z
        }
    }
}

impl ops::SubAssign for Vec3 {
    
    /// Subtracts one Vec3 from another Vec3 and re-assigns the first Vec3 to
    /// the new Vec3.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec3;
    /// # let mut some_vec3 = Vec3::new();
    /// # let some_other_vec3 = Vec3::new();
    /// some_vec3 -= some_other_vec3;
    /// ```
    fn sub_assign(&mut self, other_vec3: Vec3) {
        *self = Vec3 {
            x: self.x - other_vec3.x,
            y: self.y - other_vec3.y,
            z: self.z - other_vec3.z
        };
    }
}

#[cfg(test)]
mod tests {
    use ::Vec3;

    #[test]
    fn create_new_vec3() {
        assert_eq!(Vec3::new(), Vec3 { x: 0.0, y: 0.0, z: 0.0 })
    }

    #[test]
    fn create_new_vec3_from_values() {
        let x = 0.0;
        let y = 1.0;
        let z = 2.5;
        let vec = Vec3::new_from_values(&x, &y, &z);
        assert_eq!(vec, Vec3 { x, y, z });
    }

    #[test]
    fn create_new_vec3_from_array() {
        let input_array = [ 1.0, 2.5, 5.5 ];
        let vec = Vec3::new_from_array(&input_array);
        assert_eq!(vec, Vec3 { x: 1.0, y: 2.5, z: 5.5 });
    }

    #[test]
    fn get_vec3_as_array() {
        let vec = Vec3 { x: 1.0, y: 3.5, z: 0.5 };
        let array = vec.to_array();
        assert_eq!(array, [1.0, 3.5, 0.5])
    }

    #[test]
    fn get_cross_product() {
        let v1 = Vec3 { x: 2.0, y: 4.5, z: 0.0 };
        let v2 = Vec3 { x: 3.0, y: 1.5, z: 4.0 };
        assert_eq!(v1.cross_product(&v2), Vec3 { x: 18.0, y: -8.0, z: -10.5 })
    }

    #[test]
    fn get_vec3_length() {
        let vec3 = Vec3 { x: 2.0, y: 3.0, z: 6.0 };
        assert_eq!(vec3.length(), 7.0);
    }

    #[test]
    fn add_2_vec3s_together() {
        let v1 = Vec3 { x: 1.0, y: 0.0, z: 2.0 };
        let v2 = Vec3 { x: 0.0, y: 1.0, z: -1.0 };
        let v3 = v1 + v2;
        assert_eq!(v3, Vec3 { x: 1.0, y: 1.0, z: 1.0 });
    }

    #[test]
    fn add_assign_2_vec3s() {
        let mut v1 = Vec3 { x: 1.0, y: 0.0, z: 0.5 };
        let v2 = Vec3 { x: 0.0, y: 1.0, z: 0.5 };
        v1 += v2;
        assert_eq!(v1, Vec3 { x: 1.0, y: 1.0, z: 1.0 });
    }

    #[test]
    fn multiply_a_vec3_by_a_scalar() {
        let c = 2.0;
        let v1 = Vec3 { x: 1.0, y: 3.5, z: 7.0 };
        assert_eq!(c * v1, Vec3 { x: 2.0, y: 7.0, z: 14.0 });
    }

    #[test]
    fn get_dot_product() {
        let vec1 = Vec3 { x: 3.0, y: 2.0, z: 2.0 };
        let vec2 = Vec3 { x: 3.0, y: 4.0, z: 3.5 };
        assert_eq!(vec1 * vec2, 24.0);
    }

    #[test]
    fn get_a_negative_vec3() {
        let v1 = Vec3 { x: 0.0, y: 2.0, z: -1.0 };
        assert_eq!(-v1, Vec3 { x: 0.0, y: -2.0, z: 1.0 });
    }

    #[test]
    fn subtract_2_vec3s() {
        let v1 = Vec3 { x: 1.0, y: 2.0, z: 2.0 };
        let v2 = Vec3 { x: 1.0, y: 1.0, z: 3.0 };
        let v3 = v1 - v2;
        assert_eq!(v3, Vec3 { x: 0.0, y: 1.0, z: -1.0 });
    }

    #[test]
    fn subtract_assign_2_vec3s() {
        let mut v1 = Vec3 { x: 1.0, y: 2.0, z: 1.0 };
        let v2 = Vec3 { x: 1.0, y: 1.0, z: 1.5 };
        v1 -= v2;
        assert_eq!(v1, Vec3 { x: 0.0, y: 1.0, z: -0.5 });
    }
}
