use std::{
    fmt::{
        Display,
        Formatter,
    },
};
use crate::part_traits::{
    action_traits::ActionTypeT
};
use super::{
    Message,
    FmtResult
};

pub enum UserActionType {
    Reset
}

impl Default for UserActionType {
    fn default() -> UserActionType {
        UserActionType::Reset
    }
}

impl Display for UserActionType {
    fn fmt(&self, _f: &mut Formatter) -> FmtResult {
        Ok(())
    }
}

impl From<&str> for UserActionType {
    fn from(_s: &str) -> UserActionType {
        UserActionType::Reset
    }
}
impl From<UserActionType> for usize {
    fn from(s: UserActionType) -> Self {
        match s {
            UserActionType::Reset => 0
        }
    }
}

impl PartialEq for UserActionType {
    fn eq(&self, o: &UserActionType) -> bool {
        usize::from(*self) == usize::from(*o)
    }
}
impl Eq for UserActionType {}

impl Clone for UserActionType {
    fn clone(&self) -> UserActionType {
        *self
    }
}
impl Copy for UserActionType {}

impl ActionTypeT for UserActionType {
    type MessageThing = Message;

    fn get_list() -> Vec<UserActionType> {
        vec!(UserActionType::Reset)
    }

    fn get_help() -> Self::MessageThing {
        String::from("")
    }
}