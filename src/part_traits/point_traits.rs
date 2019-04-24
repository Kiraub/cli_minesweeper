use std::{
    fmt::Display,
};

pub trait PointT<N>: Display where
    N: Ord + Display + Clone {
    
    fn new() -> Self;
    fn get_x(&self) -> N;
    fn get_y(&self) -> N;
}

pub trait PointHandlerT<N> where
    N: Ord + Display + Clone {
    type PointThing: PointT<N> + Into<(N,N)>;

    fn get_x_min(&self) -> N;
    fn get_x_max(&self) -> N;
    fn get_y_min(&self) -> N;
    fn get_y_max(&self) -> N;
    fn get_index(&self, pos: Self::PointThing) -> N;
}