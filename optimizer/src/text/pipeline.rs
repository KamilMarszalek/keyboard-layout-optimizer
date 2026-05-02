use crate::{
    annealing::corpus::{
        Corpus, CorpusError, map_normalized_text_to_key_presses, supported_presses_from_modifier,
    },
    keyboard::modifier::Modifier,
    text::normalize::normalize_text,
};

pub fn build_corpus_from_text<const P: usize>(
    input: &str,
    modifier: &Modifier,
) -> Result<Corpus<P>, CorpusError> {
    let normalized_input = normalize_text(input);
    let supported = supported_presses_from_modifier(modifier)?;
    let presses = map_normalized_text_to_key_presses(&normalized_input, modifier);

    let corpus = Corpus::from_key_presses(supported, presses)?;
    Ok(corpus)
}
