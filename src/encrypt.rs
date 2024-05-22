use rand::{Rng, thread_rng};
use super::{cube::Cube, gen::encr_metadata::generate_char_pos};

pub fn encrypt_data<S>(plain_text: S) 
    where S: Into<String>
    {
        let plain_data: String = plain_text.into();
        let plain_data_len = plain_data.len();

        let dimension: usize = {
            match plain_data_len <= 3 {
                true => 3,
                false => {
                    let mut dimension = 3;

                    while ((f64::powi(dimension as f64, 2) * 6_f64).ceil() as usize) < plain_data_len {
                        dimension += 1;
                    }

                    dimension
                }
            }
        };

        let mut cube_string: Vec<Vec<Vec<u8>>> = vec![
            vec![
                vec![0; dimension];
                dimension
            ];
            6
        ];

        let char_pos = generate_char_pos(plain_data_len);

        plain_data.bytes().enumerate().for_each(|(index, cell_val)| {
            let curr_char_pos = char_pos[index];
            let dimension_sq = usize::pow(dimension, 2);
            let face_no = (curr_char_pos as f64 / dimension_sq as f64).floor() as usize;
            let row_no = (curr_char_pos % dimension_sq) / dimension;
            let cell_no = (curr_char_pos % dimension_sq) % dimension;

            cube_string[face_no][row_no][cell_no] = cell_val;
        });

        let encryption_cube = Cube::new(cube_string, dimension, char_pos);

        println!("{}", encryption_cube.to_string());

}