use std::collections::HashSet;

use crate::{
    keyboard::{
        key_press::KeyPress,
        modifier::{KeyPressMapper, SupportedPressesError},
    },
    text::pipeline::{map_normalized_text_to_key_presses, normalize_text},
};

/// Preprocessed key press statistics used by the cost function.
///
/// `supported_presses[i]` describes the logical key press represented by index `i`.
/// `unigrams[i]` stores how many times `presses[i]` occurred.
/// `bigrams[i][j]` stores how many times `presses[i]` was followed by `presses[j]`.
///
/// `None` values in the input sequence reset the bigram chain and are not counted.
pub struct Corpus<const P: usize> {
    pub supported_presses: [KeyPress; P],
    pub unigrams: [usize; P],
    pub bigrams: [[usize; P]; P],
    pub total_chars: usize,
    pub total_bigrams: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorpusError {
    UnsupportedKeyPress(KeyPress),
    DuplicateSupportedKeyPress(KeyPress),
    SupportedPresses(SupportedPressesError),
}

impl<const P: usize> Corpus<P> {
    /// Builds a corpus from text input and a key press mapper.
    pub fn from_text(input: &str, mapper: &impl KeyPressMapper) -> Result<Self, CorpusError> {
        let normalized_input = normalize_text(input);
        let supported = mapper.supported_presses::<P>().map_err(CorpusError::SupportedPresses)?;
        let presses = map_normalized_text_to_key_presses(&normalized_input, mapper);
        Corpus::from_key_presses(supported, presses)
    }
    /// Builds a corpus from a sequence of logical key presses.
    fn from_key_presses<I>(
        supported_presses: [KeyPress; P],
        input_presses: I,
    ) -> Result<Self, CorpusError>
    where
        I: IntoIterator<Item = Option<KeyPress>>,
    {
        let mut result = Self::empty_with_supported_presses(supported_presses)?;
        let mut previous: Option<usize> = None;
        for maybe_press in input_presses {
            let Some(press) = maybe_press else {
                previous = None;
                continue;
            };
            let current = result.index_of(press).ok_or(CorpusError::UnsupportedKeyPress(press))?;
            result.unigrams[current] += 1;
            result.total_chars += 1;

            if let Some(prev) = previous {
                result.bigrams[prev][current] += 1;
                result.total_bigrams += 1;
            }
            previous = Some(current);
        }
        Ok(result)
    }

    pub fn index_of(&self, press: KeyPress) -> Option<usize> {
        self.supported_presses.iter().position(|&p| p == press)
    }

    fn validate_unique_presses(presses: &[KeyPress; P]) -> Result<(), CorpusError> {
        let mut seen = HashSet::with_capacity(P);
        for &press in presses {
            if !seen.insert(press) {
                return Err(CorpusError::DuplicateSupportedKeyPress(press));
            }
        }
        Ok(())
    }

    fn empty_with_supported_presses(supported_presses: [KeyPress; P]) -> Result<Self, CorpusError> {
        Self::validate_unique_presses(&supported_presses)?;

        Ok(Self {
            supported_presses,
            unigrams: [0; P],
            bigrams: [[0; P]; P],
            total_chars: 0,
            total_bigrams: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        keyboard::modifier::Modifier,
        preset::{constants::US_PRESS_COUNT, qwerty_us::qwerty_us},
    };

    use super::*;
    use rstest::rstest;

    fn press(base: u8, shifted: bool) -> KeyPress {
        KeyPress { base, shifted }
    }

    #[rstest]
    #[case::empty_input(
        [press(b'a', false), press(b'b', false)],
        vec![],
        [0, 0],
        [[0, 0], [0, 0]],
        0,
        0
    )]
    #[case::single_press(
        [press(b'a', false), press(b'b', false)],
        vec![Some(press(b'b', false))],
        [0, 1],
        [[0, 0], [0, 0]],
        1,
        0
    )]
    #[case::bigrams(
        [press(b'a', false), press(b'b', false)],
        vec![Some(press(b'a', false)), Some(press(b'b', false)), Some(press(b'a', false))],
        [2, 1],
        [[0, 1], [1, 0]],
        3,
        2
    )]
    #[case::separator_resets_bigram_chain(
        [press(b'a', false), press(b'b', false)],
        vec![Some(press(b'a', false)), None, Some(press(b'b', false))],
        [1, 1],
        [[0, 0], [0, 0]],
        2,
        0
    )]
    #[case::shifted_and_unshifted_are_counted_separately(
        [press(b'a', false), press(b'a', true)],
        vec![Some(press(b'a', false)), Some(press(b'a', true))],
        [1, 1],
        [[0, 1], [0, 0]],
        2,
        1
    )]
    fn from_key_presses_counts_cases(
        #[case] supported: [KeyPress; 2],
        #[case] input: Vec<Option<KeyPress>>,
        #[case] expected_unigrams: [usize; 2],
        #[case] expected_bigrams: [[usize; 2]; 2],
        #[case] expected_total_chars: usize,
        #[case] expected_total_bigrams: usize,
    ) {
        let corpus = Corpus::from_key_presses(supported, input).unwrap();

        assert_eq!(corpus.supported_presses, supported);
        assert_eq!(corpus.unigrams, expected_unigrams);
        assert_eq!(corpus.bigrams, expected_bigrams);
        assert_eq!(corpus.total_chars, expected_total_chars);
        assert_eq!(corpus.total_bigrams, expected_total_bigrams);
    }

    #[rstest]
    #[case::duplicate_supported_press(
        [press(b'a', false), press(b'a', false)],
        vec![],
        CorpusError::DuplicateSupportedKeyPress(press(b'a', false))
    )]
    #[case::unsupported_input_press(
        [press(b'a', false), press(b'b', false)],
        vec![Some(press(b'c', true))],
        CorpusError::UnsupportedKeyPress(press(b'c', true))
    )]
    fn from_key_presses_returns_error(
        #[case] supported: [KeyPress; 2],
        #[case] input: Vec<Option<KeyPress>>,
        #[case] expected: CorpusError,
    ) {
        let result = Corpus::from_key_presses(supported, input);

        match result {
            Err(error) => assert_eq!(error, expected),
            Ok(_) => panic!("expected {expected:?}"),
        }
    }

    #[test]
    fn index_of_returns_expected_index() {
        let unshifted_a = press(b'a', false);
        let shifted_a = press(b'a', true);
        let shifted_b = press(b'b', true);
        let corpus = Corpus::from_key_presses([unshifted_a, shifted_a, shifted_b], []).unwrap();

        assert_eq!(corpus.index_of(unshifted_a), Some(0));
        assert_eq!(corpus.index_of(shifted_a), Some(1));
        assert_eq!(corpus.index_of(shifted_b), Some(2));
        assert_eq!(corpus.index_of(press(b'b', false)), None);
    }

    #[test]
    fn preset_corpus_from_text_empty_input_creates_empty_qwerty_us_corpus() {
        let corpus = qwerty_us().corpus_from_text("").unwrap();

        assert_eq!(corpus.supported_presses.len(), US_PRESS_COUNT);
        assert_eq!(corpus.total_chars, 0);
        assert_eq!(corpus.total_bigrams, 0);
        assert!(corpus.unigrams.iter().all(|&count| count == 0));
        assert!(corpus.bigrams.iter().flatten().all(|&count| count == 0));
    }

    #[rstest]
    #[case::base_and_shifted_presses(
        "aA!",
        [press(b'a', false), press(b'a', true), press(b'1', true)]
    )]
    #[case::normalizes_text_before_counting(
        "Ą a!",
        [press(b'a', true), press(b'a', false), press(b'1', true)]
    )]
    fn preset_corpus_from_text_counts_normalized_presses(
        #[case] input: &str,
        #[case] expected_sequence: [KeyPress; 3],
    ) {
        let corpus = qwerty_us().corpus_from_text(input).unwrap();
        let expected_indices =
            expected_sequence.map(|key_press| corpus.index_of(key_press).unwrap());

        assert_eq!(corpus.total_chars, 3);
        assert_eq!(corpus.total_bigrams, 2);
        for &index in &expected_indices {
            assert_eq!(corpus.unigrams[index], 1);
        }
        assert_eq!(corpus.bigrams[expected_indices[0]][expected_indices[1]], 1);
        assert_eq!(corpus.bigrams[expected_indices[1]][expected_indices[2]], 1);
    }

    #[test]
    fn from_text_wraps_supported_presses_error() {
        let modifier = Modifier::new([(b'a', b'A')]).unwrap();

        let result = Corpus::<1>::from_text("a", &modifier);

        assert!(matches!(
            result,
            Err(CorpusError::SupportedPresses(SupportedPressesError::InvalidSupportedPressCount {
                expected: 1,
                actual: 2
            }))
        ));
    }

    #[test]
    fn from_text_normalizes_and_counts_supported_key_presses() {
        let modifier = Modifier::new([(b'a', b'A')]).unwrap();

        let corpus = Corpus::<2>::from_text("Ą a!", &modifier).unwrap();

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

        let corpus = Corpus::<4>::from_text("a.b", &modifier).unwrap();

        assert_eq!(corpus.total_chars, 2);
        assert_eq!(corpus.total_bigrams, 0);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'a', false)).unwrap()], 1);
        assert_eq!(corpus.unigrams[corpus.index_of(press(b'b', false)).unwrap()], 1);
    }
}
