#![feature(test)]
extern crate test;
extern crate matriarch;

use test::Bencher;
use matriarch::Mat4;

// The optimized calculation looks slightly funky but it brings down the
// calculation time from 14ns to 9ns which is a 35% speed improvement in
// just moving out 4 values.

fn original_determinant(mat4: &Mat4) -> f32 {
    (mat4.a * mat4.f * mat4.k * mat4.p) - (mat4.a * mat4.f * mat4.l * mat4.o) - 
    (mat4.a * mat4.g * mat4.j * mat4.p) + (mat4.a * mat4.g * mat4.l * mat4.n) +
    (mat4.a * mat4.h * mat4.j * mat4.o) - (mat4.a * mat4.h * mat4.k * mat4.n) -
    (mat4.b * mat4.e * mat4.k * mat4.p) + (mat4.b * mat4.e * mat4.l * mat4.o) +
    (mat4.b * mat4.g * mat4.i * mat4.p) - (mat4.b * mat4.g * mat4.l * mat4.m) -
    (mat4.b * mat4.h * mat4.i * mat4.o) + (mat4.b * mat4.h * mat4.k * mat4.m) +
    (mat4.c * mat4.e * mat4.j * mat4.p) - (mat4.c * mat4.e * mat4.l * mat4.n) -
    (mat4.c * mat4.f * mat4.i * mat4.p) + (mat4.c * mat4.f * mat4.l * mat4.m) +
    (mat4.c * mat4.h * mat4.i * mat4.n) - (mat4.c * mat4.h * mat4.j * mat4.m) -
    (mat4.d * mat4.e * mat4.j * mat4.o) + (mat4.d * mat4.e * mat4.k * mat4.n) +
    (mat4.d * mat4.f * mat4.i * mat4.o) - (mat4.d * mat4.f * mat4.k * mat4.m) -
    (mat4.d * mat4.g * mat4.i * mat4.n) + (mat4.d * mat4.g * mat4.j * mat4.m)
}

fn optimized_determinant(mat4: &Mat4) -> f32 {
    (mat4.a * (
        (mat4.f * mat4.k * mat4.p) - (mat4.f * mat4.l * mat4.o)
        - (mat4.g * mat4.j * mat4.p) + (mat4.g * mat4.l * mat4.n)
        + (mat4.h * mat4.j * mat4.o) - (mat4.h * mat4.k * mat4.n)
    ))
    
    + (mat4.b * (
        - (mat4.e * mat4.k * mat4.p) + (mat4.e * mat4.l * mat4.o)
        + (mat4.g * mat4.i * mat4.p) - (mat4.g * mat4.l * mat4.m)
        - (mat4.h * mat4.i * mat4.o) + (mat4.h * mat4.k * mat4.m)
    ))
    
    + (mat4.c * (
        (mat4.e * mat4.j * mat4.p) - (mat4.e * mat4.l * mat4.n)
        - (mat4.f * mat4.i * mat4.p) + (mat4.f * mat4.l * mat4.m)
        + (mat4.h * mat4.i * mat4.n) - (mat4.h * mat4.j * mat4.m) 
    ))
    
    + (mat4.d * (
        - (mat4.e * mat4.j * mat4.o) + (mat4.e * mat4.k * mat4.n)
        + (mat4.f * mat4.i * mat4.o) - (mat4.f * mat4.k * mat4.m)
        - (mat4.g * mat4.i * mat4.n) + (mat4.g * mat4.j * mat4.m)
    ))
}

#[test]
fn ensure_same_output() {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    assert_eq!(original_determinant(&mat4), optimized_determinant(&mat4));
}

#[bench]
fn origin_determinant(b: &mut Bencher) {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    b.iter(|| {
        original_determinant(&mat4)
    });
}

#[bench]
fn opt_determinant(b: &mut Bencher) {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    b.iter(|| {
        optimized_determinant(&mat4)
    });
}
