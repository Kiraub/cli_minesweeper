mod part_traits;
mod parts;

use std::{
    error::Error,
    io::{
        stdout,
        stdin,
        Write,
    },
};
use parts::{
    config::Settings,
    game::Minesweeper
};

pub type Message = String;
pub type Number = usize;
pub type FmtResult = std::fmt::Result;
pub type UserInput = String;

pub fn parse_args(_args: &Vec<String>) -> Result<Settings, Box<dyn Error>> {
    Ok(Settings::default())
}

pub fn run(game_settings: Settings) -> Result<(), Box<dyn Error>> {
    let mut minesweeper_game = Minesweeper::new(game_settings);
    println!("{}", minesweeper_game.start());
    minesweeper_game.show_board();
    println!("Welcome to command-line interface minesweeper or CLI_MS for short!\n\
    Above you can see the minesweeper board. Use <Help> to list possible actions.");
    loop {
        let mut input = String::new();
        print!("action> ");
        stdout().flush().unwrap();
        if let Err(e) = stdin().read_line(&mut input) {
            println!("Read failed: {}", e);
        }
        let handle_result = match minesweeper_game.handle_input(input) {
            Ok(msg) => msg,
            Err(msg) => msg.to_string()
        };
        minesweeper_game.show_board();
        println!("\n{}\n", handle_result);
        if minesweeper_game.game_over() {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

}