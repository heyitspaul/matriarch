extern crate matriarch;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matriarch::Mat2;

// Turns out the Strassen method isn't fast enough to beat the raw
// performance of modern processors. Benchmark returns about 2x slowdown
// vs normal method.

fn original_mul(a: &Mat2, b: &Mat2) -> Mat2 {
    Mat2 {
        a: (a.a * b.a) + (a.b * b.c),
        b: (a.a * b.b) + (a.b * b.d),
        c: (a.c * b.a) + (a.d * b.c),
        d: (a.c * b.b) + (a.d * b.d),
    }
}

fn strassen_mul(a: &Mat2, b: &Mat2) -> Mat2 {
    let p1 = (a.a + a.d) * (b.a + b.d);
    let p2 = (a.c + a.d) * (b.a);
    let p3 = (a.a) * (b.b - b.d);
    let p4 = (a.d) * (-b.a + b.c);
    let p5 = (a.a + a.b) * (b.d);
    let p6 = (-a.a + a.c) * (b.a + b.b);
    let p7 = (a.b - a.d) * (b.c + b.d);

    Mat2 {
        a: p1 + p4 - p5 + p7,
        b: p3 + p5,
        c: p2 + p4,
        d: p1 + p2 - p3 + p6,
    }
}

#[test]
fn ensure_same_output() {
    let array1 = [1.5, 8.0, 2.0, 2.5];
    let array2 = [10.0, 4.0, 4.0, 10.0];
    let mat2a = Mat2::new_from_array(&array1);
    let mat2b = Mat2::new_from_array(&array2);
    assert_eq!(original_mul(&mat2a, &mat2b), strassen_mul(&mat2a, &mat2b));
}

fn origin_mul(c: &mut Criterion) {
    let array1 = [1.5, 8.0, 2.0, 2.5];
    let array2 = [10.0, 4.0, 4.0, 10.0];
    let mat2a = Mat2::new_from_array(&array1);
    let mat2b = Mat2::new_from_array(&array2);
    c.bench_function("origin_mul", |b| {
        b.iter(|| original_mul(black_box(&mat2a), black_box(&mat2b)))
    });
}

fn strassen_m(c: &mut Criterion) {
    let array1 = [1.5, 8.0, 2.0, 2.5];
    let array2 = [10.0, 4.0, 4.0, 10.0];
    let mat2a = Mat2::new_from_array(&array1);
    let mat2b = Mat2::new_from_array(&array2);
    c.bench_function("strassen_mul", |b| {
        b.iter(|| strassen_mul(black_box(&mat2a), black_box(&mat2b)))
    });
}

criterion_group!(benches, origin_mul, strassen_m);
criterion_main!(benches);
