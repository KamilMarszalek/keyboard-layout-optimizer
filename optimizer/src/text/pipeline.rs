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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::model::KeyPress;

    fn press(base: u8, shifted: bool) -> KeyPress {
        KeyPress { base, shifted }
    }

    #[test]
    fn build_corpus_from_text_normalizes_and_counts_supported_key_presses() {
        let modifier = Modifier::new([(b'a', b'A')]).unwrap();

        let corpus = build_corpus_from_text::<2>("Ą a!", &modifier).unwrap();

        assert_eq!(corpus.total_chars, 2);
        assert_eq!(corpus.total_bigrams, 1);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'a', false)).unwrap()], 1);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'a', true)).unwrap()], 1);
        assert_eq!(
            corpus.bigrams[corpus.index_of(press(b'a', true)).unwrap()]
                [corpus.index_of(press(b'a', false)).unwrap()],
            1
        );
    }

    #[test]
    fn build_corpus_from_text_unsupported_normalized_symbol_resets_bigram_chain() {
        let modifier = Modifier::new([(b'a', b'A'), (b'b', b'B')]).unwrap();

        let corpus = build_corpus_from_text::<4>("a.b", &modifier).unwrap();

        assert_eq!(corpus.total_chars, 2);
        assert_eq!(corpus.total_bigrams, 0);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'a', false)).unwrap()], 1);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'b', false)).unwrap()], 1);
    }

    #[test]
    fn build_corpus_from_text_returns_error_for_wrong_supported_press_count() {
        let modifier = Modifier::new([(b'a', b'A')]).unwrap();

        let result = build_corpus_from_text::<1>("a", &modifier);

        assert!(matches!(
            result,
            Err(CorpusError::InvalidSupportedPressCount { expected: 1, actual: 2 })
        ));
    }
}
