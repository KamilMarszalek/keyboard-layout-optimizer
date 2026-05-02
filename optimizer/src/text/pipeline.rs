use crate::{
    annealing::corpus::{
        Corpus, CorpusError, map_normalized_text_to_key_presses, supported_presses_from_modifier,
    },
    keyboard::modifier::Modifier,
    text::normalize::normalize_text,
};

#[derive(Debug)]
pub enum BuildCorpusError {
    SupportedPresses(String),
    Corpus(CorpusError),
}

impl From<CorpusError> for BuildCorpusError {
    fn from(error: CorpusError) -> Self {
        Self::Corpus(error)
    }
}

pub fn build_corpus_from_text<const P: usize>(
    input: &str,
    modifier: &Modifier,
) -> Result<Corpus<P>, BuildCorpusError> {
    let normalized_input = normalize_text(input);
    let supported =
        supported_presses_from_modifier(modifier).map_err(BuildCorpusError::SupportedPresses)?;
    let presses = map_normalized_text_to_key_presses(&normalized_input, modifier);

    let corpus = Corpus::from_key_presses(supported, presses)?;
    Ok(corpus)
}
