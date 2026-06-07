use crate::{
    keyboard::{geometry::Geometry, layout::Layout, modifier::Modifier},
    text::corpus::{Corpus, CorpusError},
};

/// Composes the physical geometry, modifier mapping, and symbol placement for a keyboard.
///
/// Presets are convenience values. The optimizer still operates on `Layout`, and
/// `Corpus` directly, while text ingestion only depends on the preset's modifier mapping.
#[derive(Clone)]
pub struct KeyboardPreset<const N: usize, const P: usize> {
    pub name: &'static str,
    pub geometry: Geometry<N>,
    pub modifier: Modifier,
    pub layout: Layout<N>,
}

impl<const N: usize, const P: usize> KeyboardPreset<N, P> {
    /// Builds a [`Corpus`] from raw text using this preset's modifier.
    ///
    /// Text is normalized before counting — see [`normalize_text`](crate::text::pipeline::normalize_text).
    pub fn corpus_from_text(&self, input: &str) -> Result<Corpus<P>, CorpusError> {
        Corpus::from_text(input, &self.modifier)
    }
}

#[cfg(test)]
mod tests {
    use crate::preset::{dvorak_us::dvorak_us, qwerty_us::qwerty_us};

    #[test]
    fn qwerty_and_dvorak_presets_decode_text_to_same_corpus() {
        let qwerty = qwerty_us().corpus_from_text("aA!").unwrap();
        let dvorak = dvorak_us().corpus_from_text("aA!").unwrap();

        assert_eq!(qwerty.supported_presses, dvorak.supported_presses);
        assert_eq!(qwerty.unigrams, dvorak.unigrams);
        assert_eq!(qwerty.bigrams, dvorak.bigrams);
        assert_eq!(qwerty.total_chars, dvorak.total_chars);
        assert_eq!(qwerty.total_bigrams, dvorak.total_bigrams);
    }
}
