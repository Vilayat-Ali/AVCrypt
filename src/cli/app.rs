use clap::{Args, Parser, Subcommand};

#[derive(Args)]
pub struct EncryptionCliArgs {
    pub plain_text: String,
}

#[derive(Args)]
pub struct DecryptionCliArgs {
    pub cipher_text: String,
    pub key: String,
}

#[derive(Parser)]
#[command(
    author = "Syed Vilayat Ali Rizvi <vilayatcodemysite@gmail.com>",
    version = "1.0.0-beta",
    about = "AVCrypt",
    long_about = "None"
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encrypt(EncryptionCliArgs),
    Decrypt(DecryptionCliArgs),
}
