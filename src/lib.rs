mod part_traits;
mod parts;

use std::{
    error::Error
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
    match minesweeper_game.start() {
        Ok(msg) => println!("{}", msg),
        Err(e_msg) => eprintln!("Problem starting game: {}", e_msg)
    };
    minesweeper_game.show_board();
    Ok(())
}

#[cfg(test)]
mod tests {

}