use avcrypt::cube::*;
use criterion::black_box;

pub fn init_cube_decryption() -> Cube {
    let mut cube_builder: CubeBuilder = CubeBuilder::new(black_box(
        std::iter::repeat_with(|| 'a')
            .take(u16::MAX as usize)
            .collect::<String>(),
    ));

    cube_builder.build()
}
