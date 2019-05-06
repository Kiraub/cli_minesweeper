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
        },
        point_traits::{
            PointT
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
        use std::cmp::Ordering;
        let mut max_width = 0;
        let mut list : Vec<(Coord, Field)> = Vec::new();
        let mut out : String = String::new();
        for key in self.field_map.keys() {
            list.push((*key,self.field_map.get(key).unwrap().clone()));
            if key.get_x() > max_width {
                max_width = key.get_x();
            }
        }
        list.sort_by(|some,other| -> Ordering {
            match (*some).0.get_y().cmp(&(*other).0.get_y()) {
                Ordering::Equal => (*some).0.get_x().cmp(&(*other).0.get_x()),
                any_order => any_order
            }
        });
        let mut list_iter = list.iter();
        while let Some(elem) = list_iter.next() {
            out.push_str(&format!(" {}", elem.1));
            if elem.0.get_x() == max_width {
                out.push('\n');
            }
        }
        write!(f, "{}", out)
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