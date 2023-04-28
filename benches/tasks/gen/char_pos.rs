use avcrypt::gen::gen_rand_pos::map_char_pos;

use criterion::black_box;

pub fn generate_and_map_char_pos() -> Vec<usize> {
    // 105 is dimension for a cube_string with 65535 characters
    let mut cube_string: Vec<Vec<Vec<u8>>> = vec![vec![vec![0; 105]; 105]; 6];
    let plain_text: String = std::iter::repeat_with(|| 'a')
        .take(u16::MAX as usize)
        .collect::<String>();
    // generating and returning character positions
    map_char_pos(black_box(&mut cube_string), black_box(&plain_text))
}
