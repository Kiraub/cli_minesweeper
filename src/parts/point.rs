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
    /// X Axis
    col: Number,
    /// Y Axis
    row: Number
}

impl Coord {
    pub fn create(col: Number, row: Number) -> Coord {
        Coord {
            col,
            row
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Column: <{}>, Row: <{}>", self.get_x(), self.get_y())
    }
}

impl From<Coord> for (Number,Number) {
    fn from(c: Coord) -> (Number, Number) {
        (c.get_x(), c.get_y())
    }
}
impl From<(Number,Number)> for Coord {
    fn from(tuple: (Number,Number)) -> Coord {
        Coord{col: tuple.0, row: tuple.1}
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
        self.col
    }

    fn get_y(&self) -> Number {
        self.row
    }
}