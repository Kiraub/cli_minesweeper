use std::{
    fmt::Display,
};

pub trait PointT<N>: Display + PartialEq where
    N: Ord + Display + Clone {

    fn new() -> Self;
    fn get_x(&self) -> N;
    fn get_y(&self) -> N;
}

pub trait PointHandler<N> where
    N: Ord + Display + Clone {
    type PointThing: PointT<N> + Into<(N,N)>;

    fn get_x_min(&self) -> N;
    fn get_x_max(&self) -> N;
    fn get_y_min(&self) -> N;
    fn get_y_max(&self) -> N;
    fn get_index(&self, pos: Self::PointThing) -> N;
    fn get_point(&self, idx: N) -> Self::PointThing;
}