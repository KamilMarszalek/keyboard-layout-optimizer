use crate::keyboard::common::AsciiChar;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub base: AsciiChar,
    pub shifted: bool,
}
