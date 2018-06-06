//! Implementation of a 2D Vector and its associated functions and methods.

use std::ops;

use Vec3;

/// A 2D Vector with elements x and y
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Returns a new Vec2 at [0, 0].
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// let vec2 = Vec2::new();
    /// ```
    pub fn new() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    /// Returns a new Vec2 using the given values for x and y.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// let x: f32 = 1.0;
    /// let y: f32 = 2.0;
    /// let vec2 = Vec2::new_from_values(&x, &y);
    /// ```
    pub fn new_from_values(x: &f32, y: &f32) -> Vec2 {
        Vec2 { x: *x, y: *y }
    }

    /// Returns a new Vec2 using the 0 and 1 indices of the given array,
    /// where [0] -> x, and [1] -> y.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// let input = [ 1.0, 2.0 ];
    /// let vec2 = Vec2::new_from_array(&input);
    /// ```
    pub fn new_from_array(input: &[f32; 2]) -> Vec2 {
        Vec2 { x: input[0], y: input[1] }
    }

    /// Returns an array of the Vec2's x and y values where x -> [0], y -> [1].
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let some_vec2 = Vec2::new();
    /// let array = some_vec2.to_array();
    /// ```
    pub fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    /// Returns the cross product of 2 Vec2s as if they were Vec3s with a z
    /// component of 0.
    /// 
    /// We technically can't actually cross-multiply 2 vectors
    /// in R^2, however we can add a z component of 0 and pretend the vectors 
    /// are in R^3 instead, which is why we get a Vec3 at the end.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// use matriarch::Vec3;
    /// 
    /// # let some_vec2 = Vec2::new();
    /// # let some_other_vec2 = Vec2::new();
    /// let vec3: Vec3 = some_vec2.cross_product(&some_other_vec2);
    /// ```
    pub fn cross_product(&self, other_vec2: &Vec2) -> Vec3 {
        Vec3 {
            // Since the formula would then be multiplying 0s for both the x 
            // and y, we can short circuit this by just passing a 0.0 to
            // x and y instead.
            x: 0.0,
            y: 0.0,
            z: (self.x * other_vec2.y) - (self.y * other_vec2.x),
        }
    }

    /// Returns the length of the Vec2.
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x.powi(2) + self.y.powi(2))
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    /// Adds one Vec2 to another Vec2 and returns a new Vec2.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let some_vec2 = Vec2::new();
    /// # let some_other_vec2 = Vec2::new();
    /// let vec2 = some_vec2 + some_other_vec2;
    /// ```
    fn add(self, other_vec2: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other_vec2.x,
            y: self.y + other_vec2.y,
        }
    }
}

impl ops::AddAssign for Vec2 {
    /// Adds one Vec2 to another Vec2 and re-assigns the first Vec2 to the
    /// new Vec2.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let mut some_vec2 = Vec2::new();
    /// # let some_other_vec2 = Vec2::new();
    /// some_vec2 += some_other_vec2;
    /// ```
    fn add_assign(&mut self, other_vec2: Vec2) {
        *self = Vec2 {
            x: self.x + other_vec2.x,
            y: self.y + other_vec2.y,
        };
    }
}

impl ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    /// Multiplies a scalar value by a Vec2 and returns a Vec2.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let some_scalar = 1.0;
    /// # let some_vec2 = Vec2::new();
    /// let scaled_vec2: Vec2 = some_scalar * some_vec2;
    /// ```
    fn mul(self, other_vec2: Vec2) -> Vec2 {
        Vec2 {
            x: self * other_vec2.x,
            y: self * other_vec2.y,
        }
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = f32;

    /// Returns the dot product of 2 Vec2s, which is a scalar floating point.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let some_vec2 = Vec2::new();
    /// # let some_other_vec2 = Vec2::new();
    /// let scalar: f32 = some_vec2 * some_other_vec2;
    /// ```
    fn mul(self, other_vec2: Vec2) -> f32 {
        (self.x * other_vec2.x) + (self.y * other_vec2.y)
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    /// Negates the values of Vec2, which in turn negates the Vec2
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let some_vec2 = Vec2::new();
    /// let negated_vec2 = -some_vec2;
    /// ```
    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    /// Subtracts one Vec2 from another Vec2 and returns a new Vec2.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let some_vec2 = Vec2::new();
    /// # let some_other_vec2 = Vec2::new();
    /// let vec2 = some_vec2 - some_other_vec2;
    /// ```
    fn sub(self, other_vec2: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other_vec2.x,
            y: self.y - other_vec2.y,
        }
    }
}

impl ops::SubAssign for Vec2 {
    /// Subtracts one Vec2 from another Vec2 and re-assigns the first Vec2 to
    /// the new Vec2.
    /// 
    /// Example:
    /// ```
    /// # use matriarch::Vec2;
    /// # let mut some_vec2 = Vec2::new();
    /// # let some_other_vec2 = Vec2::new();
    /// some_vec2 -= some_other_vec2;
    /// ```
    fn sub_assign(&mut self, other_vec2: Vec2) {
        *self = Vec2 {
            x: self.x - other_vec2.x,
            y: self.y - other_vec2.y,
        };
    }
}

#[cfg(test)]
mod tests {
    use Vec2;
    use Vec3;

    #[test]
    fn create_new_vec2() {
        assert_eq!(Vec2::new(), Vec2 { x: 0.0, y: 0.0 })
    }

    #[test]
    fn create_new_vec2_from_values() {
        let x = 0.0;
        let y = 1.0;
        let vec = Vec2::new_from_values(&x, &y);
        assert_eq!(vec, Vec2 { x, y });
    }

    #[test]
    fn create_new_vec2_from_array() {
        let input_array = [1.0, 2.5];
        let vec = Vec2::new_from_array(&input_array);
        assert_eq!(vec, Vec2 { x: 1.0, y: 2.5 });
    }

    #[test]
    fn get_vec2_as_array() {
        let vec = Vec2 { x: 1.0, y: 3.5 };
        let array = vec.to_array();
        assert_eq!(array, [1.0, 3.5])
    }

    #[test]
    fn get_cross_product() {
        let v1 = Vec2 { x: 2.0, y: 4.5 };
        let v2 = Vec2 { x: 3.0, y: 1.5 };
        assert_eq!(v1.cross_product(&v2), Vec3 { x: 0.0, y: 0.0, z: -10.5 })
    }

    #[test]
    fn get_vec2_length() {
        let vec2 = Vec2 { x: 3.0, y: 4.0 };
        assert_eq!(vec2.length(), 5.0);
    }

    #[test]
    fn add_2_vec2s_together() {
        let v1 = Vec2 { x: 1.0, y: 0.0 };
        let v2 = Vec2 { x: 0.0, y: 1.0 };
        let v3 = v1 + v2;
        assert_eq!(v3, Vec2 { x: 1.0, y: 1.0 });
    }

    #[test]
    fn add_assign_2_vec2s() {
        let mut v1 = Vec2 { x: 1.0, y: 0.0 };
        let v2 = Vec2 { x: 0.0, y: 1.0 };
        v1 += v2;
        assert_eq!(v1, Vec2 { x: 1.0, y: 1.0 });
    }

    #[test]
    fn multiply_a_vec2_by_a_scalar() {
        let c = 2.0;
        let v1 = Vec2 { x: 1.0, y: 3.5 };
        assert_eq!(c * v1, Vec2 { x: 2.0, y: 7.0 });
    }

    #[test]
    fn get_dot_product() {
        let vec1 = Vec2 { x: 3.0, y: 2.0 };
        let vec2 = Vec2 { x: 3.0, y: 4.0 };
        assert_eq!(vec1 * vec2, 17.0);
    }

    #[test]
    fn get_a_negative_vec2() {
        let v1 = Vec2 { x: 0.0, y: 2.0 };
        assert_eq!(-v1, Vec2 { x: 0.0, y: -2.0 });
    }

    #[test]
    fn subtract_2_vec2s() {
        let v1 = Vec2 { x: 1.0, y: 2.0 };
        let v2 = Vec2 { x: 1.0, y: 1.0 };
        let v3 = v1 - v2;
        assert_eq!(v3, Vec2 { x: 0.0, y: 1.0 });
    }

    #[test]
    fn subtract_assign_2_vec2s() {
        let mut v1 = Vec2 { x: 1.0, y: 2.0 };
        let v2 = Vec2 { x: 1.0, y: 1.0 };
        v1 -= v2;
        assert_eq!(v1, Vec2 { x: 0.0, y: 1.0 });
    }
}
