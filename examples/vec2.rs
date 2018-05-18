extern crate matriarch;

use matriarch::Vec2;

fn main() {
    let vec2 = Vec2::new();
    //=> Vec2 { x: 0.0, y: 0.0 }
    let new_vec2 = vec2 + Vec2 { x: 1.0, y: 3.0 };
    println!("{:?}", new_vec2);
    //=> Vec2 { x: 1.0, y: 3.0 }
}
