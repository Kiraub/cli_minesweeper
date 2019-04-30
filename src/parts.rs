pub type Message = String;
pub type Number = usize;
pub type FmtResult = std::fmt::Result;

pub mod action;
pub mod actiontype;
pub mod config;
pub mod point;
pub mod value;
pub mod field;
pub mod board;