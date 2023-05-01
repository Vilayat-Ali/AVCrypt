mod col;
mod row;

use crate::cube::{Cube, Mixable};

impl Mixable for Cube {
    fn mix(&mut self, move_list: Vec<String>) -> Result<(), String> {
        Ok(())
    }
}
