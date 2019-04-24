use std::{
    fmt::Display,
    convert::TryFrom,
};

pub trait ConfigT: Display {
    type ConfigEntryThing: Display + PartialEq + TryFrom<String>;
    type KeyThing;
    type MessageThing: Display;

    fn new() -> Self;
    fn get(key: &str) -> Result<&Self::ConfigEntryThing, Self::MessageThing>;
    fn set(key: &str, valstr: &str) -> Result<(), Self::MessageThing>;
}