use crate::keyboard::{model::KeyPress, modifier::Modifier};

pub fn map_normalized_text_to_key_presses(
    normalized: &str,
    modifier: &Modifier,
) -> impl Iterator<Item = Option<KeyPress>> {
    normalized.bytes().map(|c| modifier.key_press_of(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{keyboard::model::KeyPress, text::corpus::Corpus};

    fn press(base: u8, shifted: bool) -> KeyPress {
        KeyPress { base, shifted }
    }

    #[test]
    fn build_corpus_from_text_normalizes_and_counts_supported_key_presses() {
        let modifier = Modifier::new([(b'a', b'A')]).unwrap();

        let corpus = Corpus::<2>::build_corpus_from_text("Ą a!", &modifier).unwrap();

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

        let corpus = Corpus::<4>::build_corpus_from_text("a.b", &modifier).unwrap();

        assert_eq!(corpus.total_chars, 2);
        assert_eq!(corpus.total_bigrams, 0);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'a', false)).unwrap()], 1);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'b', false)).unwrap()], 1);
    }

    #[test]
    fn map_normalized_text_to_key_presses_maps_supported_symbols() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();

        let result: Vec<_> = map_normalized_text_to_key_presses("aA1!", &modifier).collect();

        assert_eq!(
            result,
            vec![
                Some(KeyPress { base: b'a', shifted: false }),
                Some(KeyPress { base: b'a', shifted: true }),
                Some(KeyPress { base: b'1', shifted: false }),
                Some(KeyPress { base: b'1', shifted: true }),
            ]
        );
    }

    #[test]
    fn map_normalized_text_to_key_presses_returns_none_for_unsupported_symbols() {
        let modifier = Modifier::new([(b'a', b'A'), (b'b', b'B')]).unwrap();

        let result: Vec<_> = map_normalized_text_to_key_presses("a b", &modifier).collect();

        assert_eq!(
            result,
            vec![
                Some(KeyPress { base: b'a', shifted: false }),
                None,
                Some(KeyPress { base: b'b', shifted: false }),
            ]
        );
    }
}
