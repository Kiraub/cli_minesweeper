use std::{
    fmt::Display,
};

pub trait ActionTypeT: Display + Eq + From<&'static str> + Clone + Default + Into<usize> {
    type MessageThing: Display + From<&'static str>;

    fn get_list() -> Vec<Self>;
    fn get_help() -> Self::MessageThing;
}

pub trait ActionT: Display + From<&'static str> {
    type ActionTypeThing: ActionTypeT;
    type MessageThing: Display + From<&'static str>;

    fn new() -> Self;
    fn get_type(&self) -> Self::ActionTypeThing;
    fn get_help(&self) -> Self::MessageThing;
}

pub trait ActionHandler {
    type ActionThing: ActionT;
    type ResultThing;

    /// Perform an action that may change the data of the implementor type and returns a result
    fn do_action(&mut self, action: Self::ActionThing) -> Self::ResultThing;
}