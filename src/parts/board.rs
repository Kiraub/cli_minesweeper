use std::{
    fmt::{
        Display,
        Formatter,
    },
    collections::{
        HashMap
    }
};
use crate::{
    part_traits::{
        board_traits::{
            BoardT
        },
        field_traits::{
            FieldT
        }
    },
    Message,
    FmtResult,
    Number
};
use super::{
    point::Coord,
    field::Field
};

pub struct MineBoard {
    field_map: HashMap<Coord,Field>
}

impl MineBoard {
    pub fn init(&mut self, width: Number, height: Number) -> Result<(), Message> {
        for width_c in 0..width {
            for height_c in 0..height {
                let coord = Coord::from((width_c,height_c));
                self.field_map.insert(coord, Field::new());
            }
        }
        Ok(())
    }

    fn get(&self, pos: &Coord) -> Option<&Field> {
        self.field_map.get(pos)
    }

    fn set(&mut self, pos: &Coord, field: Field) -> bool {
        if let Some(s_field) = self.field_map.get_mut(pos) {
            s_field.set_value(field.get_value());
            s_field.set_visible(field.get_visible());
            true
        } else {
            false
        }
    }
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

    fn get_field(&self, pos: &Self::PositionThing) -> Result<&Field, Self::MessageThing> {
        if let Some(field) = self.get(&pos) {
            Ok(field)
        } else {
            Err("No such field".to_string())
        }
    }

    fn set_field(&mut self, pos: &Self::PositionThing, field: Field) -> Result<(), Self::MessageThing> {
        if self.set(pos, field) {
            Ok(())
        } else {
            Err("No such field".to_string())
        }
    }
}