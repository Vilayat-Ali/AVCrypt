#![allow(unused, dead_code)]

use crate::gen::gen_rand_pos::map_char_pos;
use std::fmt::{self, Display, Formatter, Result};

/// Cube struct contains the required meta data that defined the overall structure of our cube instance.
/// The Cube struct will be implementing `Mixable` trait, denoting that cube struct can be mixed and all
/// Cube mixing algorithms are very much applicable to our Cube struct.
#[derive(Debug)]
pub struct Cube {
    /// cube_string contains the 3-d mapping of the cube structure in the memory. Where each cell contains
    /// a speicfic character in the byte format.
    pub cube_string: Vec<Vec<Vec<u8>>>,
    /// dimension tells us what is the size of the cube_string ie. 3x3 cube contains upto 54 characters
    pub dimension: usize,
    /// char_pos is a vector containing positioning of various character element of the plain string in the
    /// cube_string object.<br/>
    /// char_pos contains just ordered positions of the characters and which lets to less space by the data payload.
    pub char_pos: Vec<usize>,
}

impl Cube {
    /// new method instantiate a new Cube object in the memory.
    pub fn new(cube_string: Vec<Vec<Vec<u8>>>, dimension: usize, char_pos: Vec<usize>) -> Self {
        Self {
            cube_string,
            dimension,
            char_pos,
        }
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut face_no: usize = 0;
        let mut row_no: usize = 0;
        let mut cell_no: usize = 0;

        for face in self.cube_string.clone().iter() {
            // printing faces
            //
            // Meta data
            writeln!(f, "----------------------")?;
            writeln!(f, " Face no: {}/6", face_no)?;
            match face_no {
                1 => {
                    writeln!(f, " Face side: LEFT")?;
                }
                2 => {
                    writeln!(f, " Face side: FRONT")?;
                }
                3 => {
                    writeln!(f, " Face side: RIGHT")?;
                }
                4 => {
                    writeln!(f, " Face side: BACK")?;
                }
                5 => {
                    writeln!(f, " Face side: TOP")?;
                }
                6 => {
                    writeln!(f, " Face side: BOTTOM")?;
                }
                _ => {}
            }
            writeln!(f, "----------------------\n")?;
            // Meta data
            //
            // starting horizontal border
            {
                for x in (0..((&self.dimension * 3) + (&self.dimension + 1))) {
                    if x % 4 == 0 {
                        write!(f, "+")?;
                    } else {
                        write!(f, "-")?;
                    }
                }
                write!(f, "\n");
            }

            // Printing cube
            for row in face.into_iter() {
                // preparing row string
                let mut row_string: String =
                    row.iter().map(|x| format!("| {} ", x)).collect::<String>();

                row_string.push('|');

                writeln!(f, "{}", row_string)?;

                {
                    for x in (0..((&self.dimension * 3) + (&self.dimension + 1))) {
                        if x % 4 == 0 {
                            write!(f, "+")?;
                        } else {
                            write!(f, "-")?;
                        }
                    }
                    write!(f, "\n")?;
                }

                row_no += 1;
            }

            face_no += 1;
        }
        Ok(())
    }
}

pub struct CubeBuilder {
    pub data: String,
    pub key: Option<String>,
    pub cube_string: Option<Vec<Vec<Vec<u8>>>>,
    pub dimension: usize,
    pub char_pos: Option<usize>,
}

impl CubeBuilder {
    // new instance
    pub fn new(data: String) -> Self {
        // calculating dimension using the dimension formula
        let dimension: usize = f64::ceil(f64::sqrt((data.len() as f64) / 6_f64)) as usize;
        let dimension: usize = match dimension > 3 {
            true => dimension,
            false => 3,
        };

        Self {
            data,
            key: None,
            cube_string: Some(vec![vec![vec![0; dimension]; dimension]; 6]),
            dimension,
            char_pos: None,
        }
    }
    // setting key
    pub fn key(&mut self, key: String) {
        //! This method is used to set key in the Cube Builder.
        self.key = Some(key);
    }
    // build the cube instance
    pub fn build(&mut self) -> Cube {
        //! This method is used to generate Cube instance from inputs passed from the CubeBuilder.
        //! This build process performs two important tasks:
        //! 1. Generate random character positions and map it onto the cube_string.
        //! 2. Generate char_pos from the ransom character positions generated.

        // mapping the cube_string with random character positions
        let char_pos: Vec<usize> = map_char_pos(self.cube_string.as_mut().unwrap(), &self.data);

        Cube::new(self.cube_string.clone().unwrap(), self.dimension, char_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cube_instance_init() {
        assert_eq!(2 + 2, 4);
    }
}
