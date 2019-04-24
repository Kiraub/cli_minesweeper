use std::{
    fmt::Display,
};

pub trait BoardT<V,N>: Display where
    V: Display + Clone,
    N: Ord + Display + Clone {
    type PositionThing: Into<(N,N)>;
    type MessageThing;

    fn new() -> Self;
    fn get_value(&self, pos: Self::PositionThing) -> V;
    fn set_value(&self, pos: Self::PositionThing, val: V) -> Result<(), Self::MessageThing>;
}