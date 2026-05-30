use super::constants::{US_KEY_COUNT, US_PRESS_COUNT};
use crate::{
    keyboard::{common::AsciiChar, geometry::Geometry, layout::Layout, modifier::Modifier},
    preset::keyboard_preset::KeyboardPreset,
};

#[rustfmt::skip]
pub const QWERTY_US_SYMBOLS: [AsciiChar; US_KEY_COUNT] = [
    b'`', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=',
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\\',
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'',
    b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',
];

impl Layout<US_KEY_COUNT> {
    /// Returns the QWERTY US symbol arrangement for the main printable keys.
    pub fn qwerty_us(modifier: &Modifier) -> Self {
        Self::new(&QWERTY_US_SYMBOLS, modifier).unwrap()
    }
}

pub fn qwerty_us() -> KeyboardPreset<US_KEY_COUNT, US_PRESS_COUNT> {
    let geometry = Geometry::standard_us();
    let modifier = Modifier::standard_us();
    let layout = Layout::qwerty_us(&modifier);

    KeyboardPreset { name: "qwerty_us", geometry, modifier, layout }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::key_press::KeyPress;

    #[test]
    fn qwerty_us_composes_keyboard_and_corpus() {
        let preset = qwerty_us();

        let corpus = preset.corpus_from_text("aA!").unwrap();

        assert_eq!(preset.layout.key_of(b'a'), Some(26));
        assert!(corpus.index_of(KeyPress { base: b'a', shifted: false }).is_some());
        assert!(corpus.index_of(KeyPress { base: b'a', shifted: true }).is_some());
        assert!(corpus.index_of(KeyPress { base: b'1', shifted: true }).is_some());
    }
}
