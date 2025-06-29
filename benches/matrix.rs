use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rosemary_renderer::Matrix;

fn matrix_ops(c: &mut Criterion) {
    let three = black_box(Matrix::new_3x3([
        1.0, 5.0, 0.0,
        -3.0, 2.0, 7.0,
        0.0, 6.0, -3.0
    ]));

    let four = black_box(Matrix::new_4x4([
        -6.0, 1.0, 1.0, 6.0,
        -8.0, 6.0, 8.0, 6.0,
        -1.0, 0.0, 8.0, 2.0,
        -7.0, 1.0, -1.0, 1.0
    ]));
    
    c.bench_function("4x4 matrix submatrix",
    |b| b.iter(|| four.submatrix(black_box(2), black_box(1))));
    c.bench_function("4x4 matrix inversion",
    |b| b.iter(|| four.inverse()));
    c.bench_function("4x4 matrix determinant",
    |b| b.iter(|| four.calc_determinant()));
    c.bench_function("4x4 matrix cofactor",
    |b| b.iter(|| four.calc_cofactor(black_box(2), black_box(1))));

    c.bench_function("3x3 matrix submatrixation",
        |b| b.iter(|| (three.submatrix(black_box(0), black_box(2)))));
}

criterion_group!(benches, matrix_ops);
criterion_main!(benches);