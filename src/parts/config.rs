use std::{
    fmt::{
        Display,
        Formatter,
    }
};
use crate::part_traits::{
    config_traits::{
        ConfigT
    }
};
use super::{
    FmtResult,
    Number,
    Message
};

#[derive(PartialEq,Eq,Clone,Copy)]
enum Neighbourhood {
    /// indirect neighbours up to manhattan-2
    Moore,
    /// direct neighbours only, manhattan-1
    VonNeumann
}

impl From<Neighbourhood> for usize {
    fn from(s: Neighbourhood) -> Self {
        match s {
            Neighbourhood::Moore => 0,
            Neighbourhood::VonNeumann => 1
        }
    }
}

impl From<usize> for Neighbourhood {
    fn from(s: usize) -> Self {
        match s {
            0 => Neighbourhood::Moore,
            1 => Neighbourhood::VonNeumann,
            _ => Neighbourhood::default()
        }
    }
}

impl Default for Neighbourhood {
    fn default() -> Self {
        Neighbourhood::Moore
    }
}

pub struct Settings {
    width: Number,
    height: Number,
    neighbourhood: Neighbourhood,
    lifes: Number
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            width: 5,
            height: 5,
            neighbourhood: Neighbourhood::default(),
            lifes: 3
        }
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "debug settings display")
    }
}

impl ConfigT<Number,Number> for Settings {

    fn new() -> Self {
        Default::default()
    }

    fn get(&self, key: Number) -> Number {
        match key {
            0 => self.width,
            1 => self.height,
            2 => self.neighbourhood.into(),
            3 => self.lifes,
            _ => panic!()
        }
    }

    fn set(&mut self, key: Number, val: Number) -> Number {
        match key {
            0 => {self.width=val;self.width},
            1 => {self.height=val;self.height},
            2 => {self.neighbourhood=val.into();self.neighbourhood.into()},
            3 => {self.lifes=val;self.lifes},
            _ => panic!()
        }
    }

    fn has(&self, _key: Number) -> bool {
        true
    }

    fn can(&self, _key: Number, _val: Number) -> bool {
        true
    }
}