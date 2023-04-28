#![allow(unused)]

use criterion::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod tasks;

use tasks::{cube_init, decryption, encryption};

fn criterion_benchmark(c: &mut Criterion) {
    // ------------------------------------------------------------------------------------------
    //
    // Cube Instance Initialization
    //
    // ------------------------------------------------------------------------------------------

    // encryption cube initialization benchmark
    c.bench_function("Cube Initialization (Encryption)", |b| {
        b.iter(|| cube_init::encryption::init_cube_encryption())
    });

    // decryption cube initialization benchmark
    c.bench_function("Cube Initialization (Decryption)", |b| {
        b.iter(|| cube_init::decryption::init_cube_encryption())
    });

    //
    // ------------------------------------------------------------------------------------------
    //

    // ------------------------------------------------------------------------------------------
    //
    // Encryption
    //
    // ------------------------------------------------------------------------------------------

    //
    // ------------------------------------------------------------------------------------------
    //

    // ------------------------------------------------------------------------------------------
    //
    // Decryption
    //
    // ------------------------------------------------------------------------------------------

    //
    // ------------------------------------------------------------------------------------------
    //
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
