use std::{
    fmt::{
        Result as FmtResult,
        Display,
        Formatter,
    },
};
use crate::part_traits::{
    action_traits::{
        ActionT
    }
};
use super::{
    actiontype::{
        UserActionType
    },
    Message
};

pub struct UserAction {
    atype: UserActionType
}

impl Display for UserAction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Ok(())
    }
}

impl From<&str> for UserAction {
    fn from(s: &str) -> UserAction {
        UserAction::new()
    }
}

impl ActionT for UserAction {
    type ActionTypeThing = UserActionType;
    type MessageThing = Message;

    fn new() -> UserAction {
        let default = UserAction{
            atype: UserActionType::Reset
        };
        UserAction{..default}
    }

    fn get_type(&self) -> Self::ActionTypeThing {
        self.atype
    }

    fn get_help(&self) -> Self::MessageThing {
        Message::from("")
    }
}