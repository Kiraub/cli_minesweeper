use std::{
    fmt::Display,
};

pub trait BoardT<F,N>: Display where
    F: Display,
    N: Ord + Display + Clone {
    type PositionThing: Into<(N,N)>;
    type MessageThing;

    fn new() -> Self;
    fn get_field(&self, pos: Self::PositionThing) -> F;
    fn set_field(&self, pos: Self::PositionThing, field: F) -> Result<(), Self::MessageThing>;
}