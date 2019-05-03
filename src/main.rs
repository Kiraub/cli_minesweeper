
use std::{env, process};

use cli_minesweeper;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = cli_minesweeper::parse(&args).unwrap_or_else(|parse_err| {
        eprintln!("Problem parsing arguments: {}", parse_err);
        process::exit(1);
    });

    if let Err(runtime_err) = cli_minesweeper::run(config) {
        eprintln!("Runtime error: {}", runtime_err);
        process::exit(1);
    }

    process::exit(0);
}
