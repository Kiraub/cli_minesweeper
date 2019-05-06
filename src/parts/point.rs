use std::{
    fmt::{
        Display,
        Formatter,
    }
};
use crate::{
    part_traits::{
        point_traits::{
            PointT
        }
    },
    Number,
    FmtResult
};

#[derive(Default,Hash,Clone,Copy)]
pub struct Coord {
    row: Number,
    col: Number
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "<Row: {}, Column: {}>", self.get_x(), self.get_y())
    }
}

impl From<Coord> for (Number,Number) {
    fn from(c: Coord) -> (Number, Number) {
        (c.get_x(), c.get_y())
    }
}
impl From<(Number,Number)> for Coord {
    fn from(tuple: (Number,Number)) -> Coord {
        Coord{row: tuple.0, col: tuple.1}
    }
}

impl PartialEq for Coord {
    fn eq(&self, o: &Coord) -> bool {
        self.get_x() == o.get_x() && self.get_y() == o.get_y()
    }
}
impl Eq for Coord {}

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