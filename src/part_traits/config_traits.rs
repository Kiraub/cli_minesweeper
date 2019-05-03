use std::{
    fmt::Display
};

pub enum ConfigHandlerError {
    IllegalKeyError,
    IllegalValueError
}

pub trait ConfigT<Key,Value>: Display where
    Key: PartialEq,
    Value: Default + PartialEq {

    fn new() -> Self;

    fn get(&self, key: Key) -> Value;
    fn set(&mut self, key: Key, val: Value) -> Value;

    fn has(&self, key: Key) -> bool;
    fn can(&self, key: Key, val: Value) -> bool;
}

pub trait ConfigHandler<K,V> {
    type Key: From<K> + Into<K> + PartialEq;
    type Value: From<V> + Into<V> + PartialEq + Default;
    type ConfigThing: ConfigT<Self::Key,Self::Value>;
    type UserInputThing: Into<K> + Into<V>;
    type MessageThing: Display;

    fn try_get(config: &Self::ConfigThing, key: Self::UserInputThing) -> Result<&V,ConfigHandlerError>;
    fn try_set(config: &mut Self::ConfigThing, key: Self::UserInputThing, val: Self::UserInputThing) -> Result<&V,ConfigHandlerError>;
}