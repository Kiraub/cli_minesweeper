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
    field_map: HashMap<Coord,Field>
}

impl Display for MineBoard {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "debug board display")
    }
}

impl BoardT<Field,Number> for MineBoard {
    type PositionThing = Coord;
    type MessageThing = Message;

    fn new() -> Self {
        MineBoard{
            field_map: HashMap::new()
        }
    }

    fn get_field(&self, _pos: Self::PositionThing) -> Field {
        Field::default()
    }

    fn set_field(&self, _pos: Self::PositionThing, _field: Field) -> Result<(), Self::MessageThing> {
        Ok(())
    }
}