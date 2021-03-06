use std::{
    fmt::{
        Display,
        Formatter,
    },
    cmp::{
        Ordering
    }
};
use crate::{
    part_traits::{
        value_traits::{
            ValueT
        }
    },
    FmtResult
};

pub enum FieldValue {
    None,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Bomb
}

impl Default for FieldValue {
    fn default() -> FieldValue {
        FieldValue::None
    }
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let out : &str = self.into();
        write!(f, "{}", out)
    }
}

impl From<FieldValue> for usize {
    fn from(s: FieldValue) -> Self {
        use FieldValue::*;
        match s {
            None => 0,
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Bomb => 999
        }
    }
}
impl From<&FieldValue> for &str {
    fn from(s: &FieldValue) -> Self {
        use FieldValue::*;
        match s {
            None => "_",
            One => "1",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Bomb => "B"
        }
    }
}

impl Clone for FieldValue {
    fn clone(&self) -> FieldValue {
        *self
    }
}
impl Copy for FieldValue {}

impl PartialEq for FieldValue {
    fn eq(&self, o: &FieldValue) -> bool {
        usize::from(*self) == usize::from(*o)
    }
}
impl Eq for FieldValue {}

impl PartialOrd for FieldValue {
    fn partial_cmp(&self, o: &FieldValue) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for FieldValue {
    fn cmp(&self, o: &FieldValue) -> Ordering {
        usize::from(*self).cmp(&usize::from(*o))
    }
}

impl ValueT for FieldValue {
    fn new() -> FieldValue {
        Default::default()
    }
}