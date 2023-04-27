use rand::{rngs::ThreadRng, thread_rng, Rng};

// Notes:
// -----
//
// 1. using references inorder to reduce memory footprint.
//
// 2. using references and byte encoding for stringified data, hence reduxing deep copying of data objects
//    in the memory

pub fn map_char_pos(cube_string: &mut Vec<Vec<Vec<u8>>>, data: &String) -> Vec<usize> {
    let dimension: usize = cube_string[0].len().pow(2); // dimension of the cube_string
    let mut char_pos: Vec<usize> = Vec::with_capacity(dimension * 6);
    let mut rng: ThreadRng = thread_rng();
    let mut placeholder_vec: Vec<usize> = (0..(dimension * 6))
        .map(|x: usize| x)
        .collect::<Vec<usize>>(); // generating placeholder vector

    for char_byte in data.as_bytes() {
        // generating uniformly distributed random value for placeholder vector index
        let rand_idx: usize = rng.gen_range(0..placeholder_vec.len());
        let current_rand_pos: usize = placeholder_vec[rand_idx];
        // pushing it in the char_pos vector
        char_pos.push(current_rand_pos);

        //
        // ------------------------------------------------------------
        //
        // mapping the values into the cube_string
        //
        // ------------------------------------------------------------
        //

        // face number
        let face_no: usize = (current_rand_pos / dimension.pow(2)) as usize;
        // row number
        let row_no: usize = {
            let remainder: usize = current_rand_pos % (dimension.pow(2) as usize);
            remainder / dimension
        };
        // cell number
        let cell_no: usize = current_rand_pos % dimension;

        // mapping onto the cube string
        cube_string[face_no][row_no][cell_no] = *char_byte;

        // removing the element in the 'rand_idx' position in the placeholder vec to avoid collision
        placeholder_vec.swap_remove(rand_idx); // swap remove inorder to make removal to be O(1)
    }

    char_pos
}
