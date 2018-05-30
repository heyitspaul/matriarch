#![feature(test)]
extern crate test;
extern crate matriarch;

use test::Bencher;
use matriarch::Mat4;

// The optimized calculation looks slightly funky but it brings down the
// calculation time from 14ns to 9ns which is a 35% speed improvement in
// just moving out 4 values.
//
// The further optimized calculation looks even funkier, but calculating the
// determinant of a Mat4 takes only 6ns. That's a speed improvement of 33% over
// the already optimized algorithm, and an overall improvement of 57%
// over the original full fat calculation. The multiplication count has also
// decreased from 72 multiplications to 52 multiplications to now only 40.

// Benchmark Output:
// running 4 tests
// test ensure_same_output ... ignored
// test extra_opt_determinant ... bench:           6 ns/iter (+/- 0)
// test opt_determinant       ... bench:           9 ns/iter (+/- 0)
// test origin_determinant    ... bench:          14 ns/iter (+/- 0)

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

fn further_optimized_determinant(mat4: &Mat4) -> f32 {
    (mat4.a * (
        (mat4.p * ( (mat4.f * mat4.k) - (mat4.g * mat4.j) ) )
        + (mat4.o * ( - (mat4.f * mat4.l) + (mat4.h * mat4.j) ) )
        + (mat4.n * ( (mat4.g * mat4.l) - (mat4.h * mat4.k) ) )
    ))
    
    + (mat4.b * (
        (mat4.p * ( - (mat4.e * mat4.k) + (mat4.g * mat4.i) ) )
        + (mat4.o * ( (mat4.e * mat4.l) - (mat4.h * mat4.i) ) )
        + (mat4.m * ( - (mat4.g * mat4.l) + (mat4.h * mat4.k) ) )
    ))
    
    + (mat4.c * (
        (mat4.p * ( (mat4.e * mat4.j) - (mat4.f * mat4.i) ) )
        + (mat4.n * ( - (mat4.e * mat4.l) + (mat4.h * mat4.i) ) )
        + (mat4.m * ( (mat4.f * mat4.l) - (mat4.h * mat4.j) ) )
    ))
    
    + (mat4.d * (
        (mat4.o * ( - (mat4.e * mat4.j) + (mat4.f * mat4.i) ) )
        + (mat4.n * ( (mat4.e * mat4.k) - (mat4.g * mat4.i) ) )
        + (mat4.m * ( - (mat4.f * mat4.k) + (mat4.g * mat4.j) ) )
    ))
}

#[test]
fn ensure_same_output() {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    assert_eq!(original_determinant(&mat4), optimized_determinant(&mat4));
    assert_eq!(original_determinant(&mat4), further_optimized_determinant(&mat4));
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

#[bench]
fn extra_opt_determinant(b: &mut Bencher) {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    b.iter(|| {
        further_optimized_determinant(&mat4)
    });
}
