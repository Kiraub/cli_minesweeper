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
    vis: bool,
    mark: bool
}

impl Field {
    pub fn _debug(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Field<Value: <{}>; Visible: <{}>; Marked: <{}>>", self.get_value(), self.get_visible(), self.get_marked())
    }

    pub fn bomb() -> Field {
        Field {
            val: FieldValue::Bomb,
            ..Field::default()
        }
    }

    pub fn is_bomb(&self) -> bool {
        self.val == FieldValue::Bomb
    }

    pub fn is_empty(&self) -> bool {
        self.val == FieldValue::None
    }

    pub fn get_marked(&self) -> bool {
        self.mark
    }

    pub fn set_marked(&mut self, mark: bool) {
        self.mark = mark
    }
    
    pub fn incr(&mut self) {
        use FieldValue::*;
        match self.val {
            None => self.val = One,
            One => self.val = Two,
            Two => self.val = Three,
            Three => self.val = Four,
            Four => self.val = Five,
            Five => self.val = Six,
            Six => self.val = Seven,
            Seven => self.val = Eight,
            Eight => panic!(),
            Bomb => ()
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            val: FieldValue::default(),
            vis: false,
            mark: false
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.get_visible() {
            write!(f, "{}", self.get_value())
        } else if self.get_marked() {
            write!(f, "M")
        } else {
            write!(f, "?")
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