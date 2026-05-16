use super::constants::{DVORAK_US_SYMBOLS, QWERTY_US_SYMBOLS, US_KEY_COUNT};
use crate::keyboard::{layout::Layout, modifier::Modifier};

impl Layout<US_KEY_COUNT> {
    /// Returns the QWERTY US symbol arrangement for the main printable keys.
    pub fn qwerty_us(modifier: &Modifier) -> Self {
        Self::new(&QWERTY_US_SYMBOLS, modifier).unwrap()
    }

    /// Returns the Dvorak US symbol arrangement for the main printable keys.
    pub fn dvorak_us(modifier: &Modifier) -> Self {
        Self::new(&DVORAK_US_SYMBOLS, modifier).unwrap()
    }

    /// Compatibility wrapper for the old preset-specific name.
    pub fn standard_us() -> Self {
        let modifier = Modifier::standard_us();
        Self::qwerty_us(&modifier)
    }
}
