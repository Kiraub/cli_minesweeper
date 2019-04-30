use std::{
    fmt::{
        Display,
        Formatter,
    }
};
use crate::part_traits::{
    field_traits::{
        FieldT
    }
};
use super::{
    FmtResult,
    value::FieldValue
};

pub struct Field {
    val: FieldValue,
    vis: bool
}

impl Default for Field {
    fn default() -> Self {
        Field {
            val: FieldValue::default(),
            vis: false
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "<Value: {}, Visible: {}>", self.get_value(), self.get_visible())
    }
}

impl FieldT<FieldValue> for Field {

    fn new() -> Self {
        Default::default()
    }
    
    fn get_value(&self) -> FieldValue {
        self.val
    }

    fn get_visible(&self) -> bool {
        self.vis
    }

    fn set_visible(&mut self, vis: bool) {
        self.vis = vis;
    }
}