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

pub fn parse(_args: &Vec<String>) -> Result<Settings, Box<dyn Error>> {
    Ok(Settings::default())
}

pub fn run(game_settings: Settings) -> Result<(), Box<dyn Error>> {
    let mut minesweeper_game = Minesweeper::new(game_settings);
    println!("{}", match minesweeper_game.start() {
        Ok(msg) => msg.to_string(),
        Err(e_msg) => e_msg
    });
    Ok(())
}

#[cfg(test)]
mod tests {

}