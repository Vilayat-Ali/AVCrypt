mod col;
mod row;

use clap::error::ErrorKind;

pub fn mix_cube(cube_string: &mut Vec<Vec<Vec<u8>>>) -> Result<(), ErrorKind> {
    print!("{:?}", cube_string);
    Ok(())
}
