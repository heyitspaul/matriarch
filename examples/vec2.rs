extern crate matriarch;

use matriarch::Vec2;

fn main() {
    addition();
    //=> Vec2 { x: 3.0, y: 5.5 }
    subtraction();
    //=> Vec2 { x: 1.0, y: -0.5 }
    scalar_product();
    //=> Vec2 { x: 5.0, y: 6.25 }
    dot_product();
    //=> 9.5
    cross_product();
    //=> Vec3 { x: 0.0, y: 0.0, z: 3.5 }
}

fn addition() {
    let vec2 = Vec2 { x: 2.0, y: 2.5 };
    let other_vec2 = Vec2 { x: 1.0, y: 3.0 };
    let new_vec2 = vec2 + other_vec2;
    println!("Addition:");
    println!("  {:?}", vec2);
    println!("+ {:?}", other_vec2);
    println!("-------------------------");
    println!("= {:?}\n", new_vec2);
}

fn subtraction() {
    let vec2 = Vec2 { x: 2.0, y: 2.5 };
    let other_vec2 = Vec2 { x: 1.0, y: 3.0 };
    let new_vec2 = vec2 - other_vec2;
    println!("Subtraction:");
    println!("  {:?}", vec2);
    println!("- {:?}", other_vec2);
    println!("-------------------------");
    println!("= {:?}\n", new_vec2);
}

fn scalar_product() {
    let vec2 = Vec2 { x: 2.0, y: 2.5 };
    let scalar: f32 = 2.5;
    let new_vec2 = scalar * vec2;
    println!("Scalar Product:");
    println!("  {:?}", scalar);
    println!("* {:?}", vec2);
    println!("-------------------------");
    println!("= {:?}\n", new_vec2);
}

fn dot_product() {
    let vec2 = Vec2 { x: 2.0, y: 2.5 };
    let other_vec2 = Vec2 { x: 1.0, y: 3.0 };
    let new_vec2 = vec2 * other_vec2;
    println!("Dot Product:");
    println!("  {:?}", vec2);
    println!("* {:?}", other_vec2);
    println!("-------------------------");
    println!("= {:?}\n", new_vec2);
}

fn cross_product() {
    let vec2 = Vec2 { x: 2.0, y: 2.5 };
    let other_vec2 = Vec2 { x: 1.0, y: 3.0 };
    let new_vec3 = vec2.cross_product(&other_vec2);
    println!("Cross Product:");
    println!("  {:?}", vec2);
    println!("× {:?}", other_vec2);
    println!("---------------------------------");
    println!("= {:?}\n", new_vec3);
}
