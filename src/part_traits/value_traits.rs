use std::{
    fmt::Display,
};

pub trait ValueT: Display + PartialEq {
    
    fn new() -> Self;
}