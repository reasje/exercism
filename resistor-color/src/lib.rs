use core::fmt;
use enum_iterator::{ Sequence ,all};
use int_enum::{IntEnum ,};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum , Sequence  )]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    // because the color that is given should be a valid 
    // resistor color then the we are sure that the convention
    // to the usize is also always valid .
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {

    match ResistorColor::from_int(value as u8) {
        // the to string functionality is brought to you by 
        // impl fmt::Display .
        Ok(value) => value.to_string(),
        Err(_) => String::from("value out of range"),
    }

}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<ResistorColor>>()
}
