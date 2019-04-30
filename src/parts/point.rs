use std::{
    fmt::{
        Display,
        Formatter,
    }
};
use crate::part_traits::{
    point_traits::{
        PointT
    }
};
use super::{
    FmtResult,
    Number
};

#[derive(Default)]
pub struct Coord {
    row: Number,
    col: Number
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "<Row: {}, Column: {}>", self.get_x(), self.get_y())
    }
}

impl From<(Number,Number)> for Coord {
    fn from(tuple: (Number, Number)) -> Self {
        Coord{row: tuple.0, col: tuple.1}
    }
}

impl PointT<Number> for Coord {

    fn new() -> Self {
        Default::default()
    }

    fn get_x(&self) -> Number {
        self.row
    }

    fn get_y(&self) -> Number {
        self.col
    }
}