use core::fmt;

use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Sequence, Clone, Copy, IntEnum)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<ResistorColor> for String {
    fn from(color: ResistorColor) -> Self {
        format!("{}", color)
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

// i liked nix3r's solution, so implementing that!
pub fn value_to_color_string(value: u32) -> String {
    // // option 1 - using if let
    // if let Ok(color) = ResistorColor::from_int(value) {
    //     format!("{:?}", color)
    // } else {
    //     String::from("value out of range")
    // }

    // option 2 - using match
    // match ResistorColor::from_int(value) {
    //     Ok(color) => format!("{:?}", color),
    //     Err(_) => String::from("value out of range"),
    // }

    // option 3 - using nix3r's solution
    // this is probably less performant though, since we call
    //  String::from, which calls fmt which finally calls the
    //  {:?} debug implementation
    match ResistorColor::from_int(value) {
        Ok(color) => color.into(),             // String::from(color),
        Err(_) => "value out of range".into(), // String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // all() enumerates the variants in the order they are listed in the definition
    //  rather than their int values. so, putting Black = 0 in 2nd row means
    //  Black will be added as 2nd element of the vector

    // no need to explicitly specify the type for collect (using collect::<Vec<_>>)
    //  as Rust can figure that out from the function's return type.
    all::<ResistorColor>().collect()
}
