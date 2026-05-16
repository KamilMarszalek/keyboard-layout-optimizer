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
