use std::env;
use avcrypt::encrypt::encrypt_data;

enum Operation {
    Encr,
    Decr,
}

fn parse_args() -> (Operation, Vec<String>) {
    let mut args = env::args().skip(1).collect::<Vec<String>>();

    let operation = match args.get(0).map(String::as_str) {
        Some("encrypt") => Operation::Encr,
        Some("decrypt") => Operation::Decr,
        _ => {
            eprintln!("Invalid arguments passed!");
            std::process::exit(1);
        }
    };

    args.remove(0);
    (operation, args)
}

fn main() {
    let (operation, args) = parse_args();

    match operation {
        Operation::Encr => {
            let plain_txt = args.get(0).unwrap_or_else(|| {
                eprintln!("No plain string passed!");
                std::process::exit(1);
            });

            // encryption process
            encrypt_data(plain_txt);
        }
        Operation::Decr => {
            if args.len() < 2 {
                eprintln!("No cube string or moves passed!");
                std::process::exit(1);
            }

            let cube_string = &args[0];
            let moves = &args[1];

            // decryption process
            println!("cube string: {}\nmoves: {}", cube_string, moves);
        }
    }
}
