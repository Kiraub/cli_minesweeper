use std::{
    fmt::{
        Display,
        Formatter,
    },
    collections::{
        HashMap
    }
};
use crate::part_traits::{
    board_traits::{
        BoardT
    }
};
use super::{
    FmtResult,
    Number,
    Message,
    point::Coord,
    field::Field
};

pub struct MineBoard {
    fieldMap: HashMap<Coord,Field>
}

impl BoardT<Field,Number> for MineBoard {
    type PositionThing = Coord;
    type MessageThing = Message;

    fn new() -> Self {
        MineBoard{
            fieldMap: HashMap::new()
        }
    }
}