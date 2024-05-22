#![allow(unused, dead_code)]
use std::{
    fmt::{self, Display, Formatter},
    string::ParseError,
};

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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut face_no: usize = 1;
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
                let mut row_string: String = row
                    .into_iter()
                    .map(|x| match x.to_string().len() {
                        1 => {
                            format!("| {} ", x)
                        }
                        2 => {
                            format!("| {}", x)
                        }
                        3 => {
                            format!("|{}", x)
                        }
                        _ => format!(""),
                    })
                    .collect::<String>();

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
