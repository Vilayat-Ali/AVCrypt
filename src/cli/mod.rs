use avcrypt::cube::{Cube, CubeBuilder};
use clap::Parser;

mod app;

use app::{Cli, Commands};

pub fn run_cli() {
    // cli instance
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(data) => {
            // performing encryption
            let mut cube_builder: CubeBuilder = CubeBuilder::new(data.plain_text);
            println!("Dimension: {}", cube_builder.dimension);
            let cube: Cube = cube_builder.build();
            println!("{}", cube.to_string());
        }
        Commands::Decrypt(data) => {
            // performing decryption
            println!("Hello World");
        }
    }
}
