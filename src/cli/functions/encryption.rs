use avcrypt::cube::{Cube, CubeBuilder};
use std::fmt::Error;

pub fn perform_encryption(plain_text: String) -> Result<(), Error> {
    // performing encryption
    let mut cube_builder: CubeBuilder = CubeBuilder::new(plain_text);
    let cube: Cube = cube_builder.build();
    println!("{}", cube.to_string());
    Ok(())
}
