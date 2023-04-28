use crate::cube::{Cube, CubeBuilder};
use clap::Parser;

mod app;
mod functions;

use app::{Cli, Commands};
use functions::*;

pub fn run_cli() {
    // cli instance
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(data) => {
            // performing encryption
            let mut cube_builder: CubeBuilder = CubeBuilder::new(data.plain_text);
            let cube: Cube = cube_builder.build();
            println!("Cube: {:#?}", cube);
            println!("{}", cube.to_string());
        }
        Commands::Decrypt(data) => {
            // performing decryption
            println!("Hello World");
        }
    }
}
