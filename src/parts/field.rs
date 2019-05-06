use std::{
    fmt::{
        Display,
        Formatter,
    }
};
use crate::{
    part_traits::{
        field_traits::{
            FieldT
        }
    },
    FmtResult
};
use super::{
    value::FieldValue
};

#[derive(Clone)]
pub struct Field {
    val: FieldValue,
    vis: bool
}

impl Field {
    pub fn _debug(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "<Value: {}, Visible: {}>", self.get_value(), self.get_visible())
    }
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
        if self.get_visible() {
            write!(f, "{}", self.get_value())
        } else {
            write!(f, "#")
        }
    }
}

impl FieldT<FieldValue> for Field {

    fn new() -> Self {
        Default::default()
    }
    
    fn get_value(&self) -> FieldValue {
        self.val
    }

    fn set_value(&mut self, val: FieldValue) {
        self.val = val;
    }

    fn get_visible(&self) -> bool {
        self.vis
    }

    fn set_visible(&mut self, vis: bool) {
        self.vis = vis;
    }
}