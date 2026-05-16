use super::constants::{US_KEY_COUNT, US_PRESS_COUNT};
use crate::{
    keyboard::{
        common::AsciiChar, geometry::Geometry, layout::Layout, model::Keyboard, modifier::Modifier,
    },
    preset::keyboard_preset::KeyboardPreset,
};

#[rustfmt::skip]
pub const DVORAK_US_SYMBOLS: [AsciiChar; US_KEY_COUNT] = [
    b'`', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'[', b']',
    b'\'', b',', b'.', b'p', b'y', b'f', b'g', b'c', b'r', b'l', b'/', b'=', b'\\',
    b'a', b'o', b'e', b'u', b'i', b'd', b'h', b't', b'n', b's', b'-',
    b';', b'q', b'j', b'k', b'x', b'b', b'm', b'w', b'v', b'z',
];

impl Layout<US_KEY_COUNT> {
    /// Returns the Dvorak US symbol arrangement for the main printable keys.
    pub fn dvorak_us(modifier: &Modifier) -> Self {
        Self::new(&DVORAK_US_SYMBOLS, modifier).unwrap()
    }
}

impl Keyboard<US_KEY_COUNT> {
    /// Returns a keyboard using ANSI US geometry and Dvorak US symbol placement.
    pub fn dvorak_us() -> Self {
        let geometry = Geometry::standard_us();
        let modifier = Modifier::standard_us();
        let layout = Layout::dvorak_us(&modifier);
        Self::new(geometry, layout)
    }
}

pub fn dvorak_us() -> KeyboardPreset<US_KEY_COUNT, US_PRESS_COUNT> {
    let geometry = Geometry::standard_us();
    let modifier = Modifier::standard_us();
    let layout = Layout::dvorak_us(&modifier);

    KeyboardPreset { name: "dvorak_us", geometry, modifier, layout }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::model::KeyPress;

    #[test]
    fn dvorak_us_reuses_us_ascii_modifier_with_different_symbol_placement() {
        let preset = dvorak_us();

        let keyboard = preset.keyboard();
        let corpus = preset.corpus_from_text("aA!").unwrap();

        assert_eq!(keyboard.layout.key_of(b'a'), Some(26));
        assert_eq!(keyboard.layout.key_of(b'o'), Some(27));
        assert_eq!(keyboard.layout.key_of(b's'), Some(35));
        assert!(corpus.index_of(KeyPress { base: b'a', shifted: false }).is_some());
        assert!(corpus.index_of(KeyPress { base: b'1', shifted: true }).is_some());
    }
}
