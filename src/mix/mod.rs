mod col;
mod row;

use crate::cube::Cube;

pub trait Mixable {
    fn mix(&mut self, move_list: Vec<String>) -> Result<(), String>;
}

impl Mixable for Cube {
    fn mix(&mut self, move_list: Vec<String>) -> Result<(), String> {
        Ok(())
    }
}
