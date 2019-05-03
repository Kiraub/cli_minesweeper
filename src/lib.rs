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

pub fn run(mut game_settings: Settings) -> Result<(), Box<dyn Error>> {
    let minesweeper_game = Minesweeper::new();
    Ok(())
}

#[cfg(test)]
mod tests {

}