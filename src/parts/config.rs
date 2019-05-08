use std::{
    fmt::{
        Display,
        Formatter,
    }
};
use crate::{
    part_traits::{
        config_traits::{
            ConfigT
        },
        action_traits::{
            ActionHandler
        }
    },
    FmtResult,
    Number,
    Message
};
use super::{
    action::UserAction
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
    bombs: Number
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            width: 15,
            height: 15,
            neighbourhood: Neighbourhood::default(),
            bombs: 25
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
            3 => self.bombs,
            _ => panic!()
        }
    }

    fn set(&mut self, key: Number, val: Number) {
        match key {
            0 => self.width=val,
            1 => self.height=val,
            2 => self.neighbourhood=val.into(),
            3 => self.bombs=val,
            _ => panic!()
        };
    }

    fn has(&self, key: Number) -> bool {
        [0,1,2,3].contains(&key)
    }

    fn can(&self, key: Number, val: Number) -> bool {
        match key {
            0 => 5 <= val && val <= 20,
            1 => 5 <= val && val <= 20,
            2 => val == 0 || val == 1,
            3 => 1 <= val && val <= (self.width*self.height),
            _ => true
        }
    }
}

impl ActionHandler for Settings {
    type ActionThing = UserAction;
    type ResultThing = Result<bool,Message>;

    fn do_action(&mut self, _action: Self::ActionThing) -> Self::ResultThing {
        Ok(true)
    }
}