use std::{
    fmt::Display,
};

pub trait FieldT<V>: Display where
    V: Display + PartialEq + Clone {

    fn new() -> Self;
    fn get_value(&self) -> V;
    fn set_value(&mut self, val: V);
    fn get_visible(&self) -> bool;
    fn set_visible(&mut self, vis: bool);
}