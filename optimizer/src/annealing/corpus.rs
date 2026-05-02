use crate::keyboard::model::KeyPress;

/// Preprocessed text statistics used by the cost function.
///
/// This type is a skeleton for the final metric implementation and may be
/// refined once text preprocessing and modifier handling are finalized.
pub struct Corpus<const N: usize> {
    pub presses: [KeyPress; N],
    pub unigrams: [usize; N],
    pub bigrams: [[usize; N]; N],
    pub total_chars: usize,
    pub total_bigrams: usize,
}
