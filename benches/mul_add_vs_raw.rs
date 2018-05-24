#![feature(test)]
extern crate test;
extern crate rand;
extern crate matriarch;

use test::Bencher;
use rand::Rng;
use matriarch::Mat4;

fn raw_mul_add(first_mat4: Mat4, mat4: Mat4) -> Mat4 {
    Mat4 {
        a: (first_mat4.a * mat4.a) + (first_mat4.b * mat4.e) + (first_mat4.c * mat4.i) + (first_mat4.d * mat4.m),
        b: (first_mat4.a * mat4.b) + (first_mat4.b * mat4.f) + (first_mat4.c * mat4.j) + (first_mat4.d * mat4.n),
        c: (first_mat4.a * mat4.c) + (first_mat4.b * mat4.g) + (first_mat4.c * mat4.k) + (first_mat4.d * mat4.o),
        d: (first_mat4.a * mat4.d) + (first_mat4.b * mat4.h) + (first_mat4.c * mat4.l) + (first_mat4.d * mat4.p),
        e: (first_mat4.e * mat4.a) + (first_mat4.f * mat4.e) + (first_mat4.g * mat4.i) + (first_mat4.h * mat4.m),
        f: (first_mat4.e * mat4.b) + (first_mat4.f * mat4.f) + (first_mat4.g * mat4.j) + (first_mat4.h * mat4.n),
        g: (first_mat4.e * mat4.c) + (first_mat4.f * mat4.g) + (first_mat4.g * mat4.k) + (first_mat4.h * mat4.o),
        h: (first_mat4.e * mat4.d) + (first_mat4.f * mat4.h) + (first_mat4.g * mat4.l) + (first_mat4.h * mat4.p),
        i: (first_mat4.i * mat4.a) + (first_mat4.j * mat4.e) + (first_mat4.k * mat4.i) + (first_mat4.l * mat4.m),
        j: (first_mat4.i * mat4.b) + (first_mat4.j * mat4.f) + (first_mat4.k * mat4.j) + (first_mat4.l * mat4.n),
        k: (first_mat4.i * mat4.c) + (first_mat4.j * mat4.g) + (first_mat4.k * mat4.k) + (first_mat4.l * mat4.o),
        l: (first_mat4.i * mat4.d) + (first_mat4.j * mat4.h) + (first_mat4.k * mat4.l) + (first_mat4.l * mat4.p),
        m: (first_mat4.m * mat4.a) + (first_mat4.n * mat4.e) + (first_mat4.o * mat4.i) + (first_mat4.p * mat4.m),
        n: (first_mat4.m * mat4.b) + (first_mat4.n * mat4.f) + (first_mat4.o * mat4.j) + (first_mat4.p * mat4.n),
        o: (first_mat4.m * mat4.c) + (first_mat4.n * mat4.g) + (first_mat4.o * mat4.k) + (first_mat4.p * mat4.o),
        p: (first_mat4.m * mat4.d) + (first_mat4.n * mat4.h) + (first_mat4.o * mat4.l) + (first_mat4.p * mat4.p),
    }
}

// https://doc.rust-lang.org/std/primitive.f32.html#method.mul_add
// The documentation implies that this should have better performance, but in
// reality there seems to be a massive slowdown over the original method of raw
// multiplication and addition.
fn opt_mul_add(first_mat4: Mat4, mat4: Mat4) -> Mat4 {
    Mat4 {
        a: first_mat4.a.mul_add(mat4.a, first_mat4.b.mul_add(mat4.e, first_mat4.c.mul_add(mat4.i, first_mat4.d * mat4.m))),
        b: first_mat4.a.mul_add(mat4.b, first_mat4.b.mul_add(mat4.f, first_mat4.c.mul_add(mat4.j, first_mat4.d * mat4.n))),
        c: (first_mat4.a * mat4.c) + (first_mat4.b * mat4.g) + (first_mat4.c * mat4.k) + (first_mat4.d * mat4.o),
        d: (first_mat4.a * mat4.d) + (first_mat4.b * mat4.h) + (first_mat4.c * mat4.l) + (first_mat4.d * mat4.p),
        e: (first_mat4.e * mat4.a) + (first_mat4.f * mat4.e) + (first_mat4.g * mat4.i) + (first_mat4.h * mat4.m),
        f: (first_mat4.e * mat4.b) + (first_mat4.f * mat4.f) + (first_mat4.g * mat4.j) + (first_mat4.h * mat4.n),
        g: (first_mat4.e * mat4.c) + (first_mat4.f * mat4.g) + (first_mat4.g * mat4.k) + (first_mat4.h * mat4.o),
        h: (first_mat4.e * mat4.d) + (first_mat4.f * mat4.h) + (first_mat4.g * mat4.l) + (first_mat4.h * mat4.p),
        i: (first_mat4.i * mat4.a) + (first_mat4.j * mat4.e) + (first_mat4.k * mat4.i) + (first_mat4.l * mat4.m),
        j: (first_mat4.i * mat4.b) + (first_mat4.j * mat4.f) + (first_mat4.k * mat4.j) + (first_mat4.l * mat4.n),
        k: (first_mat4.i * mat4.c) + (first_mat4.j * mat4.g) + (first_mat4.k * mat4.k) + (first_mat4.l * mat4.o),
        l: (first_mat4.i * mat4.d) + (first_mat4.j * mat4.h) + (first_mat4.k * mat4.l) + (first_mat4.l * mat4.p),
        m: (first_mat4.m * mat4.a) + (first_mat4.n * mat4.e) + (first_mat4.o * mat4.i) + (first_mat4.p * mat4.m),
        n: (first_mat4.m * mat4.b) + (first_mat4.n * mat4.f) + (first_mat4.o * mat4.j) + (first_mat4.p * mat4.n),
        o: (first_mat4.m * mat4.c) + (first_mat4.n * mat4.g) + (first_mat4.o * mat4.k) + (first_mat4.p * mat4.o),
        p: (first_mat4.m * mat4.d) + (first_mat4.n * mat4.h) + (first_mat4.o * mat4.l) + (first_mat4.p * mat4.p),
    }
}

#[test]
fn ensure_same_output() {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    let iden = Mat4::new_from_array(&array);
    assert_eq!(raw_mul_add(mat4, iden), opt_mul_add(mat4, iden));
}

#[bench]
fn raw_matrix_multiplication(b: &mut Bencher) {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    let other_mat4 = Mat4::identity();
    b.iter(||{
        raw_mul_add(mat4, other_mat4);
    });
}

#[bench]
fn muladd_matrix_multiplication(b: &mut Bencher) {
    let array = [ 1.5, 8.0, 2.0, 2.5, 10.0, 4.0, 4.0, 10.0, 3.5, 6.0, 7.0, 0.0, 7.0, 4.0, 2.0, 1.0 ];
    let mat4 = Mat4::new_from_array(&array);
    let other_mat4 = Mat4::identity();
    b.iter(|| {
        opt_mul_add(mat4, other_mat4);
    });
}
