use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

//use std::convert::TryFrom;
//#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Sequence, IntEnum)]
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

pub fn color_to_value(_color: ResistorColor) -> u32 {
    //unimplemented!("convert a color into a numerical representation")
    _color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    /* unimplemented!(
        "convert the value {} into a string representation of color",
        value
    ) */

    match ResistorColor::from_int(value as u16) {
        Ok(value) => {
            return format!("{:?}", value);
        }
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    //unimplemented!("return a list of all the colors ordered by resistance")
    all::<ResistorColor>().collect()
}
