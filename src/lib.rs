mod part_traits;
mod parts;

use std::{
    error::Error
};
use parts::{
    config::Settings
};

pub fn parse(_args: &Vec<String>) -> Result<Settings, Box<dyn Error>> {
    Ok(Settings::default())
}

pub fn run(mut game_settings: Settings) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {

}