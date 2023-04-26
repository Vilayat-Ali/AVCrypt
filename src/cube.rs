#![allow(unused, dead_code)]

//! Cube struct contains the required meta data that defined the overall structure of our cube instance.
//! The Cube struct will be implementing `Mixable` trait, denoting that cube struct can be mixed and all
//! Cube mixing algorithms are very much applicable to our Cube struct.
pub struct Cube<T> {
    /// cube_string contains the 3-d mapping of the cube structure in the memory. Where each cell contains
    /// a speicfic character in the byte format.
    pub cube_string: Vec<Vec<u8>>,
    /// dimension tells us what is the size of the cube_string ie. 3x3 cube contains upto 54 characters
    pub dimension: u64,
    /// char_pos is a vector containing positioning of various character element of the plain string in the
    /// cube_string object
    pub char_pos: Vec<(u8, T)>,
}

struct CubeBuilder {
    initial_string: String,
}

pub trait Mixable {}
