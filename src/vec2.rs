use std::ops;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {

    /// Returns a new Vec2 at [0, 0].
    pub fn new() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    /// Returns a new Vec2 using the given values for x and y.
    pub fn new_from_values(x: &f32, y: &f32) -> Vec2 {
        Vec2 { x: *x, y: *y }
    }

    /// Returns a new Vec2 using the 0 and 1 indices of the given array,
    /// where [0] -> x, and [1] -> y.
    pub fn new_from_array(input: &[f32]) -> Vec2 {
        Vec2 { x: input[0], y: input[1] }
    }

    /// Returns an array of the Vec2's x and y values where x -> [0], y -> [1].
    pub fn to_array(&self) -> [f32; 2] {
        [ self.x, self.y ]
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    /// Adds one Vec2 to another Vec2 and returns a new Vec2.
    fn add(self, other_vec2: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other_vec2.x,
            y: self.y + other_vec2.y
        }
    }
}

impl ops::AddAssign for Vec2 {

    /// Adds one Vec2 to another Vec2 and re-assigns the first Vec2 to the
    /// new Vec2.
    fn add_assign(&mut self, other_vec2: Vec2) {
        *self = Vec2 {
            x: self.x + other_vec2.x,
            y: self.y + other_vec2.y
        };
    }
}

impl ops::Neg for Vec2 {
    type Output = Vec2;

    /// Negates the values of Vec2, which in turn negates the Vec2
    fn neg(self) -> Vec2 {
        return Vec2{ x: -self.x, y: -self.y }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    /// Subtracts one Vec2 from another Vec2 and returns a new Vec2.
    fn sub(self, other_vec2: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other_vec2.x,
            y: self.y - other_vec2.y
        }
    }
}

impl ops::SubAssign for Vec2 {

    /// Subtracts one Vec2 from another Vec2 and re-assigns the first Vec2 to
    /// the new Vec2.
    fn sub_assign(&mut self, other_vec2: Vec2) {
        *self = Vec2 {
            x: self.x - other_vec2.x,
            y: self.y - other_vec2.y
        };
    }
}

#[cfg(test)]
mod tests {
    use vec2::Vec2;

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
        let input_array = [ 1.0, 2.5];
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
