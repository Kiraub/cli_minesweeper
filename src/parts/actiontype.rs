use std::{
    fmt::{
        Display,
        Formatter,
    },
};
use crate::{
    part_traits::{
        action_traits::ActionTypeT
    },
    Message,
    FmtResult
};

pub enum UserActionType {
    Unknown,
    Reset,
    Set,
    Mark,
    Quit,
    Pick
}

impl Default for UserActionType {
    fn default() -> UserActionType {
        UserActionType::Unknown
    }
}

impl Display for UserActionType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", usize::from(*self))
    }
}

impl From<&str> for UserActionType {
    fn from(s: &str) -> UserActionType {
        match s {
            "Reset" => UserActionType::Reset,
            "Set" => UserActionType::Set,
            "Mark" => UserActionType::Mark,
            "Quit" => UserActionType::Quit,
            "Pick" => UserActionType::Pick,
            _ => UserActionType::Unknown
        }
    }
}
impl From<UserActionType> for usize {
    fn from(s: UserActionType) -> Self {
        match s {
            UserActionType::Unknown => 999,
            UserActionType::Reset => 0,
            UserActionType::Set => 1,
            UserActionType::Mark => 2,
            UserActionType::Quit => 3,
            UserActionType::Pick => 4
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
        vec!(
            UserActionType::Pick,
            UserActionType::Mark,
            UserActionType::Reset,
            UserActionType::Set,
            UserActionType::Quit
        )
    }

    fn get_help() -> Self::MessageThing {
        String::from("")
    }
}