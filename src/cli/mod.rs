use clap::Parser;

mod app;
mod errors;
mod functions;

use app::{Cli, Commands};
use functions::{decryption::perform_decryption, encryption::perform_encryption};

pub fn run_cli() {
    // cli instance
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(data) => match perform_encryption(data.plain_text) {
            Ok(_encrypted_data) => {}
            Err(_e) => {}
        },
        Commands::Decrypt(data) => match perform_decryption(data.cipher_text) {
            Ok(_encrypted_data) => {}
            Err(_e) => {}
        },
    }
}
