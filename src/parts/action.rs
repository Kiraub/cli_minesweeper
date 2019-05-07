use std::{
    fmt::{
        Display,
        Formatter,
    },
};
use crate::{
    part_traits::{
        action_traits::{
            ActionT
        }
    },
    Message,
    FmtResult
};
use super::{
    actiontype::{
        UserActionType
    }
};

pub struct UserAction {
    atype: UserActionType,
    args: Vec<String>
}

impl UserAction {
    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

impl Default for UserAction {
    fn default() -> Self {
        UserAction {
            atype: UserActionType::default(),
            args: Vec::new()
        }
    }
}

impl Display for UserAction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Action<Atype<{}>; Args<{:?}>>", self.atype, self.args)
    }
}

impl From<&str> for UserAction {
    fn from(s: &str) -> UserAction {
        let mut split = s.split_whitespace();
        let atype : UserActionType;
        let mut args : Vec<String> = Vec::new();
        if let Some(typestr) = split.next() {
            atype = UserActionType::from(typestr);
        } else {
            atype = UserActionType::default();
        }
        while let Some(argstr) = split.next() {
            args.push(String::from(argstr));
        }
        UserAction {
            atype,
            args
        }
    }
}

impl ActionT for UserAction {
    type ActionTypeThing = UserActionType;
    type MessageThing = Message;

    fn new() -> UserAction {
        UserAction{..Default::default()}
    }

    fn get_type(&self) -> Self::ActionTypeThing {
        self.atype
    }

    fn get_help(&self) -> Self::MessageThing {
        Message::from("")
    }
}