use super::constants::{US_KEY_COUNT, US_PRESS_COUNT};
use crate::{
    keyboard::{geometry::Geometry, layout::Layout, model::Keyboard, modifier::Modifier},
    text::corpus::{Corpus, CorpusError},
};

/// Composes the physical geometry, modifier mapping, and symbol placement for a keyboard.
///
/// Presets are convenience values. The optimizer still operates on `Keyboard`, `Layout`, and
/// `Corpus` directly, while text ingestion only depends on the preset's modifier mapping.
#[derive(Clone)]
pub struct KeyboardPreset<const N: usize, const P: usize> {
    pub name: &'static str,
    pub geometry: Geometry<N>,
    pub modifier: Modifier,
    pub layout: Layout<N>,
}

impl<const N: usize, const P: usize> KeyboardPreset<N, P> {
    pub fn keyboard(&self) -> Keyboard<N> {
        Keyboard::new(self.geometry.clone(), self.layout.clone())
    }

    pub fn corpus_from_text(&self, input: &str) -> Result<Corpus<P>, CorpusError> {
        Corpus::from_text(input, &self.modifier)
    }
}

pub fn qwerty_us() -> KeyboardPreset<US_KEY_COUNT, US_PRESS_COUNT> {
    let geometry = Geometry::standard_us();
    let modifier = Modifier::standard_us();
    let layout = Layout::qwerty_us(&modifier);

    KeyboardPreset { name: "qwerty_us", geometry, modifier, layout }
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
    fn qwerty_us_composes_keyboard_and_corpus() {
        let preset = qwerty_us();

        let keyboard = preset.keyboard();
        let corpus = preset.corpus_from_text("aA!").unwrap();

        assert_eq!(keyboard.layout.key_of(b'a'), Some(26));
        assert!(corpus.index_of(KeyPress { base: b'a', shifted: false }).is_some());
        assert!(corpus.index_of(KeyPress { base: b'a', shifted: true }).is_some());
        assert!(corpus.index_of(KeyPress { base: b'1', shifted: true }).is_some());
    }

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
