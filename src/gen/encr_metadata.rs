use rand::{thread_rng, Rng};

pub fn generate_char_pos(data_len: usize) -> Vec<usize> {
    let mut placeholder_vec = (0..data_len).collect::<Vec<usize>>();
    let mut char_pos: Vec<usize> = Vec::new();
    let mut rng = thread_rng();

    while placeholder_vec.len() != 0 {
        let random_pos = rng.gen_range(0..placeholder_vec.len());
        char_pos.push(placeholder_vec[random_pos]);
        placeholder_vec.swap_remove(random_pos);
    }
    
    char_pos
}