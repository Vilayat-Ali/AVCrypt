#![allow(unused)]

use criterion::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod tasks;

use tasks::cube_init::{decryption, encryption};

fn criterion_benchmark(c: &mut Criterion) {
    // encryption cube initialization benchmark
    c.bench_function("Cube Initialization (Encryption)", |b| {
        b.iter(|| encryption::init_cube_encryption())
    });

    // decryption cube initialization benchmark
    c.bench_function("Cube Initialization (Decryption)", |b| {
        b.iter(|| decryption::init_cube_encryption())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
