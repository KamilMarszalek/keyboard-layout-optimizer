use crate::{
    keyboard::{
        model::KeyPress,
        modifier::{Modifier, SupportedPressesError},
    },
    text::pipeline::{map_normalized_text_to_key_presses, normalize_text},
};

const STANDARD_US_PRESS_COUNT: usize = 94;

/// Preprocessed key press statistics used by the cost function.
///
/// `presses[i]` describes the logical key press represented by index `i`.
/// `unigrams[i]` stores how many times `presses[i]` occurred.
/// `bigrams[i][j]` stores how many times `presses[i]` was followed by `presses[j]`.
///
/// `None` values in the input sequence reset the bigram chain and are not counted.
pub struct Corpus<const P: usize> {
    pub presses: [KeyPress; P],
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
    /// Builds a corpus from text input and modifier
    pub fn from_text(input: &str, modifier: &Modifier) -> Result<Self, CorpusError> {
        let normalized_input = normalize_text(input);
        let supported = modifier.supported_presses().map_err(CorpusError::SupportedPresses)?;
        let presses = map_normalized_text_to_key_presses(&normalized_input, modifier);
        Corpus::from_key_presses(supported, presses)
    }
    /// Builds a corpus from a sequence of logical key presses.
    pub fn from_key_presses<I>(
        supported_presses: [KeyPress; P],
        input_presses: I,
    ) -> Result<Self, CorpusError>
    where
        I: IntoIterator<Item = Option<KeyPress>>,
    {
        Self::validate_unique_presses(&supported_presses)?;
        let mut result = Self {
            presses: supported_presses,
            unigrams: [0; P],
            bigrams: [[0; P]; P],
            total_chars: 0,
            total_bigrams: 0,
        };
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
        self.presses.iter().position(|&p| p == press)
    }

    fn validate_unique_presses(presses: &[KeyPress; P]) -> Result<(), CorpusError> {
        for i in 0..P {
            for j in (i + 1)..P {
                if presses[i] == presses[j] {
                    return Err(CorpusError::DuplicateSupportedKeyPress(presses[i]));
                }
            }
        }
        Ok(())
    }
}

impl Corpus<STANDARD_US_PRESS_COUNT> {
    pub fn from_text_standard_us(input: &str) -> Result<Self, CorpusError> {
        let modifier = Modifier::standard_us();
        Self::from_text(input, &modifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn press(base: u8, shifted: bool) -> KeyPress {
        KeyPress { base, shifted }
    }

    #[test]
    fn empty_input_creates_empty_corpus() {
        let supported = [press(b'a', false), press(b'b', false)];

        let corpus = Corpus::from_key_presses(supported, []).unwrap();

        assert_eq!(corpus.presses, supported);
        assert_eq!(corpus.unigrams, [0, 0]);
        assert_eq!(corpus.bigrams, [[0, 0], [0, 0]]);
        assert_eq!(corpus.total_chars, 0);
        assert_eq!(corpus.total_bigrams, 0);
    }

    #[test]
    fn single_press_counts_unigram_without_bigram() {
        let corpus = Corpus::from_key_presses(
            [press(b'a', false), press(b'b', false)],
            [Some(press(b'b', false))],
        )
        .unwrap();

        assert_eq!(corpus.unigrams, [0, 1]);
        assert_eq!(corpus.bigrams, [[0, 0], [0, 0]]);
        assert_eq!(corpus.total_chars, 1);
        assert_eq!(corpus.total_bigrams, 0);
    }

    #[test]
    fn counts_bigrams() {
        let a = press(b'a', false);
        let b = press(b'b', false);
        let corpus = Corpus::from_key_presses([a, b], [Some(a), Some(b), Some(a)]).unwrap();

        assert_eq!(corpus.unigrams, [2, 1]);
        assert_eq!(corpus.bigrams, [[0, 1], [1, 0]]);
        assert_eq!(corpus.total_chars, 3);
        assert_eq!(corpus.total_bigrams, 2);
    }

    #[test]
    fn none_prevents_bigram_across_separator() {
        let a = press(b'a', false);
        let b = press(b'b', false);
        let corpus = Corpus::from_key_presses([a, b], [Some(a), None, Some(b)]).unwrap();

        assert_eq!(corpus.unigrams, [1, 1]);
        assert_eq!(corpus.bigrams, [[0, 0], [0, 0]]);
        assert_eq!(corpus.total_chars, 2);
        assert_eq!(corpus.total_bigrams, 0);
    }

    #[test]
    fn duplicate_supported_press_returns_error() {
        let duplicated = press(b'a', false);

        let result = Corpus::from_key_presses([duplicated, duplicated], []);

        assert!(matches!(
            result,
            Err(CorpusError::DuplicateSupportedKeyPress(found)) if found == duplicated
        ));
    }

    #[test]
    fn unsupported_input_press_returns_error() {
        let unsupported = press(b'c', true);

        let result =
            Corpus::from_key_presses([press(b'a', false), press(b'b', false)], [Some(unsupported)]);

        assert!(matches!(
            result,
            Err(CorpusError::UnsupportedKeyPress(found)) if found == unsupported
        ));
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
    fn shifted_and_unshifted_presses_are_counted_separately() {
        let unshifted_a = press(b'a', false);
        let shifted_a = press(b'a', true);

        let corpus = Corpus::from_key_presses(
            [unshifted_a, shifted_a],
            [Some(unshifted_a), Some(shifted_a)],
        )
        .unwrap();

        assert_eq!(corpus.unigrams, [1, 1]);
        assert_eq!(corpus.bigrams, [[0, 1], [0, 0]]);
        assert_eq!(corpus.total_chars, 2);
        assert_eq!(corpus.total_bigrams, 1);
    }

    #[test]
    fn from_text_standard_us_empty_input_creates_empty_standard_us_corpus() {
        let corpus = Corpus::from_text_standard_us("").unwrap();

        assert_eq!(corpus.presses.len(), STANDARD_US_PRESS_COUNT);
        assert_eq!(corpus.total_chars, 0);
        assert_eq!(corpus.total_bigrams, 0);
        assert!(corpus.unigrams.iter().all(|&count| count == 0));
        assert!(corpus.bigrams.iter().flatten().all(|&count| count == 0));
    }

    #[test]
    fn from_text_standard_us_counts_base_and_shifted_presses() {
        let corpus = Corpus::from_text_standard_us("aA!").unwrap();
        let unshifted_a = corpus.index_of(press(b'a', false)).unwrap();
        let shifted_a = corpus.index_of(press(b'a', true)).unwrap();
        let shifted_1 = corpus.index_of(press(b'1', true)).unwrap();

        assert_eq!(corpus.total_chars, 3);
        assert_eq!(corpus.total_bigrams, 2);
        assert_eq!(corpus.unigrams[unshifted_a], 1);
        assert_eq!(corpus.unigrams[shifted_a], 1);
        assert_eq!(corpus.unigrams[shifted_1], 1);
        assert_eq!(corpus.bigrams[unshifted_a][shifted_a], 1);
        assert_eq!(corpus.bigrams[shifted_a][shifted_1], 1);
    }

    #[test]
    fn from_text_standard_us_normalizes_text_before_counting() {
        let corpus = Corpus::from_text_standard_us("Ą a!").unwrap();
        let shifted_a = corpus.index_of(press(b'a', true)).unwrap();
        let unshifted_a = corpus.index_of(press(b'a', false)).unwrap();
        let shifted_1 = corpus.index_of(press(b'1', true)).unwrap();

        assert_eq!(corpus.total_chars, 3);
        assert_eq!(corpus.total_bigrams, 2);
        assert_eq!(corpus.unigrams[shifted_a], 1);
        assert_eq!(corpus.unigrams[unshifted_a], 1);
        assert_eq!(corpus.unigrams[shifted_1], 1);
        assert_eq!(corpus.bigrams[shifted_a][unshifted_a], 1);
        assert_eq!(corpus.bigrams[unshifted_a][shifted_1], 1);
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
