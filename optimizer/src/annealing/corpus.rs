use crate::keyboard::{model::KeyPress, modifier::Modifier};

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
    InvalidSupportedPressCount { expected: usize, actual: usize },
    MissingBaseKeyPress { base: u8 },
    MissingShiftedKeyPress { base: u8, shifted: u8 },
    MissingShiftMapping { base: u8 },
}

impl<const P: usize> Corpus<P> {
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

pub fn supported_presses_from_modifier<const P: usize>(
    modifier: &Modifier,
) -> Result<[KeyPress; P], CorpusError> {
    let mut key_presses = Vec::new();

    for &base in modifier.base_symbols() {
        let base_press =
            modifier.key_press_of(base).ok_or(CorpusError::MissingBaseKeyPress { base })?;

        key_presses.push(base_press);

        let shifted =
            modifier.shift(base).map_err(|_| CorpusError::MissingShiftMapping { base })?;

        let shifted_press = modifier
            .key_press_of(shifted)
            .ok_or(CorpusError::MissingShiftedKeyPress { base, shifted })?;

        key_presses.push(shifted_press);
    }

    key_presses.try_into().map_err(|key_presses: Vec<KeyPress>| {
        CorpusError::InvalidSupportedPressCount { expected: P, actual: key_presses.len() }
    })
}

pub fn map_normalized_text_to_key_presses(
    normalized: &str,
    modifier: &Modifier,
) -> impl Iterator<Item = Option<KeyPress>> {
    normalized.bytes().map(|c| modifier.key_press_of(c))
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
    fn supported_presses_from_modifier_builds_base_and_shifted_presses_in_order() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();

        let result = supported_presses_from_modifier::<4>(&modifier).unwrap();

        assert_eq!(
            result,
            [
                KeyPress { base: b'a', shifted: false },
                KeyPress { base: b'a', shifted: true },
                KeyPress { base: b'1', shifted: false },
                KeyPress { base: b'1', shifted: true },
            ]
        );
    }

    #[test]
    fn supported_presses_from_modifier_returns_error_for_wrong_size() {
        let modifier = Modifier::new([(b'a', b'A'), (b'1', b'!')]).unwrap();

        let result = supported_presses_from_modifier::<3>(&modifier);

        assert!(matches!(
            result,
            Err(CorpusError::InvalidSupportedPressCount { expected: 3, actual: 4 })
        ));
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
