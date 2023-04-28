#![allow(unused)]

use criterion::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod tasks;

use tasks::{
    cube_init::{
        decryption_cube_instance::init_cube_decryption,
        encryption_cube_instance::init_cube_encryption,
    },
    decryption, encryption,
    gen::char_pos::generate_and_map_char_pos,
};

fn criterion_benchmark(c: &mut Criterion) {
    // ------------------------------------------------------------------------------------------
    //
    // Cube Instance Initialization
    //
    // ------------------------------------------------------------------------------------------

    // encryption cube initialization benchmark
    c.bench_function("Cube Initialization (Encryption)", |b: &mut Bencher| {
        b.iter(|| init_cube_encryption())
    });

    // decryption cube initialization benchmark
    c.bench_function("Cube Initialization (Decryption)", |b: &mut Bencher| {
        b.iter(|| init_cube_encryption())
    });

    //
    // ------------------------------------------------------------------------------------------
    //

    // ------------------------------------------------------------------------------------------
    //
    // Cube Meta Data Generation
    //
    // ------------------------------------------------------------------------------------------

    c.bench_function("Generating Character Positions", |b: &mut Bencher| {
        b.iter(|| generate_and_map_char_pos())
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
