use crate::keyboard::model::KeyPress;

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
}

impl<const P: usize> Corpus<P> {
    /// Builds a corpus from a sequence of logical key presses.
    ///
    /// `supported_presses` defines the index space used by `unigrams` and `bigrams`.
    /// Each `Some(KeyPress)` in `input_presses` is counted as a character occurrence.
    /// `None` values are treated as separators: they are not counted and they reset
    /// the previous key press, so no bigram is created across them.
    ///
    /// # Errors
    ///
    /// Returns [`CorpusError::DuplicateSupportedKeyPress`] if `supported_presses`
    /// contains duplicates.
    /// Returns [`CorpusError::UnsupportedKeyPress`] if the input contains a key press
    /// that is not present in `supported_presses`.
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
