use crate::keyboard::common::AsciiChar;

/// A logical key press: the base symbol of the key and whether Shift is held.
///
/// Two presses on the same physical key are distinct if one is shifted — e.g.,
/// `a` (`shifted: false`) and `A` (`shifted: true`) are separate `KeyPress` values
/// that both refer to `base: b'a'`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub base: AsciiChar,
    pub shifted: bool,
}
