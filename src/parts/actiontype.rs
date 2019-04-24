use std::{
    fmt::{
        Result as FmtResult,
        Display,
        Formatter,
    },
};

use crate::part_traits::{
    action_traits::ActionTypeT
};

use super::{
    Message
};

pub enum UserActionType {
    Reset
}

impl Display for UserActionType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Ok(())
    }
}

impl From<&str> for UserActionType {
    fn from(s: &str) -> UserActionType {
        UserActionType::Reset
    }
}

impl PartialEq for UserActionType {
    fn eq(&self, o: &UserActionType) -> bool {
        match *self {
            o => true,
            _ => false
        }
    }
}
impl Eq for UserActionType {}

impl Copy for UserActionType {}
impl Clone for UserActionType {
    fn clone(&self) -> UserActionType {
        *self
    }
}

impl ActionTypeT for UserActionType {
    type MessageThing = Message;

    fn get_list() -> Vec<UserActionType> {
        vec!(UserActionType::Reset)
    }

    fn get_help() -> Self::MessageThing {
        String::from("")
    }
}