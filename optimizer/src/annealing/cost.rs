use crate::{
    keyboard::{
        geometry::{Coordinates, Key, Row},
        model::Keyboard,
    },
    text::corpus::Corpus,
};

/// Weights assigned to individual ergonomic metrics.
///
/// These weights are used to combine separate metric values into a single
/// scalar cost.
#[derive(Clone, Copy, Debug)]
pub struct MetricWeights {
    pub same_finger_bigrams: f64,
    pub finger_distance: f64,
    pub home_row_usage: f64,
    pub hand_alternation: f64,
    pub row_jumping: f64,
}

impl Default for MetricWeights {
    fn default() -> Self {
        Self {
            same_finger_bigrams: 1.0,
            finger_distance: 1.0,
            home_row_usage: 1.0,
            hand_alternation: 1.0,
            row_jumping: 1.0,
        }
    }
}

/// Values of individual ergonomic metrics for a keyboard layout.
///
/// This struct stores metric values before they are combined into one weighted
/// cost. Keeping the breakdown separate makes it possible to show detailed
/// statistics in the frontend.
#[derive(Clone, Copy, Debug, Default)]
pub struct MetricBreakdown {
    pub same_finger_bigrams: f64,
    pub finger_distance: f64,
    pub home_row_usage: f64,
    pub hand_alternation: f64,
    pub row_jumping: f64,
}

impl MetricBreakdown {
    /// Combines metric values into a single weighted cost.
    ///
    /// The current formula is:
    ///
    /// `SFB * w1 + FD * w2 - HRU * w3 - HA * w4 + RJ * w5`
    ///
    /// `home_row_usage` and `hand_alternation` are subtracted because larger
    /// values are considered better.
    pub fn weighted_cost(&self, weights: &MetricWeights) -> f64 {
        weights.same_finger_bigrams * self.same_finger_bigrams
            + weights.finger_distance * self.finger_distance
            - weights.home_row_usage * self.home_row_usage
            - weights.hand_alternation * self.hand_alternation
            + weights.row_jumping * self.row_jumping
    }
}

/// Weighted ergonomic cost function.
///
/// `WeightedCost` combines a text corpus and user-provided metric weights.
/// It can evaluate a keyboard layout and return either a single scalar cost
/// or a detailed metric breakdown.
#[allow(dead_code)]
pub struct WeightedCost<const N: usize, const P: usize> {
    weights: MetricWeights,
    corpus: Corpus<P>,
}

impl<const N: usize, const P: usize> WeightedCost<N, P> {
    /// Creates a new weighted cost function from metric weights and corpus statistics.
    pub fn new(weights: MetricWeights, corpus: Corpus<P>) -> Self {
        Self { weights, corpus }
    }

    /// Evaluates the weighted cost of a keyboard.
    ///
    /// Lower values are considered better by optimization algorithms.
    pub fn evaluate(&self, keyboard: &Keyboard<N>) -> f64 {
        self.evaluate_breakdown(keyboard).weighted_cost(&self.weights)
    }

    /// Computes all ergonomic metric values for a keyboard.
    ///
    /// This method returns individual metric components before they are
    /// combined into a single scalar cost.
    pub fn evaluate_breakdown(&self, keyboard: &Keyboard<N>) -> MetricBreakdown {
        MetricBreakdown {
            finger_distance: self.finger_distance(keyboard),
            home_row_usage: self.home_row_usage(keyboard),
            same_finger_bigrams: self.same_finger_bigrams(keyboard),
            hand_alternation: self.hand_alternation(keyboard),
            row_jumping: self.row_jumping(keyboard),
        }
    }

    fn finger_distance(&self, keyboard: &Keyboard<N>) -> f64 {
        let max_distance = keyboard.geometry.max_fingers_distance();
        if max_distance <= 0.0 {
            return 0.0;
        }
        let distance = Coordinates::euclidean_distance;
        self.bigrams_metric(
            keyboard,
            |_, _| true,
            |count, a, b| {
                let cost = match a.finger_assignment == b.finger_assignment {
                    true => distance(a.coords, b.coords) / max_distance,
                    false => {
                        let a_default = keyboard.geometry.default_key(a.finger_assignment).unwrap();
                        let b_default = keyboard.geometry.default_key(b.finger_assignment).unwrap();
                        (distance(a.coords, a_default.coords)
                            + distance(b.coords, b_default.coords))
                            / (2.0 * max_distance)
                    }
                };
                count as f64 * cost
            },
        )
    }

    fn home_row_usage(&self, keyboard: &Keyboard<N>) -> f64 {
        self.unigram_metric(keyboard, |a| a.row == Row::Home, |count, _| count as f64)
    }

    fn same_finger_bigrams(&self, keyboard: &Keyboard<N>) -> f64 {
        self.bigrams_metric(
            keyboard,
            |a, b| a.finger_assignment == b.finger_assignment,
            |count, _, _| count as f64,
        )
    }

    fn hand_alternation(&self, keyboard: &Keyboard<N>) -> f64 {
        self.bigrams_metric(
            keyboard,
            |a, b| a.finger_assignment.hand != b.finger_assignment.hand,
            |count, _, _| count as f64,
        )
    }

    fn row_jumping(&self, keyboard: &Keyboard<N>) -> f64 {
        self.bigrams_metric(
            keyboard,
            |a, b| a.row.order().abs_diff(b.row.order()) > 1,
            |count, _, _| count as f64,
        )
    }

    fn unigram_metric<F, W>(&self, keyboard: &Keyboard<N>, predicate: F, extractor: W) -> f64
    where
        F: Fn(&Key) -> bool,
        W: Fn(usize, &Key) -> f64,
    {
        if self.corpus.total_chars == 0 {
            return 0.0;
        }

        let key_assignments = self.key_assignments(keyboard);
        let mut metric = 0.0;

        for (idx, count) in self.corpus.unigrams.iter().enumerate() {
            let Some(object) = key_assignments[idx] else {
                continue;
            };

            if predicate(object) {
                metric += extractor(*count, object);
            }
        }

        metric / self.corpus.total_chars as f64
    }

    fn bigrams_metric<F, W>(&self, keyboard: &Keyboard<N>, predicate: F, extractor: W) -> f64
    where
        F: Fn(&Key, &Key) -> bool,
        W: Fn(usize, &Key, &Key) -> f64,
    {
        if self.corpus.total_bigrams == 0 {
            return 0.0;
        }

        let key_assignments = self.key_assignments(keyboard);
        let mut metric = 0.0;

        for (first_idx, second_chars) in self.corpus.bigrams.iter().enumerate() {
            let Some(first) = key_assignments[first_idx] else {
                continue;
            };

            for (second_idx, &count) in second_chars.iter().enumerate() {
                let Some(second) = key_assignments[second_idx] else {
                    continue;
                };

                if predicate(first, second) {
                    metric += extractor(count, first, second);
                }
            }
        }

        metric / self.corpus.total_bigrams as f64
    }

    fn key_assignments<'a>(&self, keyboard: &'a Keyboard<N>) -> [Option<&'a Key>; P] {
        std::array::from_fn(|idx| {
            let press = self.corpus.supported_presses[idx];
            let key_idx = keyboard.layout.key_of(press.base)?;
            keyboard.geometry.key(key_idx)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::qwerty_us::qwerty_us;
    use rstest::rstest;

    const TOL: f64 = 1e-12;

    #[rstest]
    #[case::empty_corpus("")]
    #[case::single_char("a")]
    #[case::same_key_repeat("aa")]
    #[case::different_fingers("qw")]
    #[case::same_finger("1q")]
    #[case::short_word("hello")]
    #[case::longer_text("the quick brown fox jumps over the lazy dog")]
    fn finger_distance_is_normalized(#[case] input: &str) {
        let preset = qwerty_us();
        let keyboard = preset.keyboard();
        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input).unwrap());

        let actual = cost.finger_distance(&keyboard);

        assert!(
            (0.0..=1.0).contains(&actual),
            "input {input:?}: expected value in [0,1], got {actual}"
        );
    }

    #[rstest]
    #[case::same_finger_farther_than_closer("qz", "1q", true)]
    #[case::same_finger_farther_than_different_finger("qz", "qw", true)]
    #[case::different_finger_farther_than_resting("qw", "as", true)]
    #[case::both_on_resting_keys_equal("as", "df", false)]
    fn finger_distance_relations(#[case] input1: &str, #[case] input2: &str, #[case] result: bool) {
        let preset = qwerty_us();
        let keyboard = preset.keyboard();
        let cost1 =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input1).unwrap());
        let cost2 =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input2).unwrap());

        let d1 = cost1.finger_distance(&keyboard);
        let d2 = cost2.finger_distance(&keyboard);

        assert_eq!((d1 > d2), result, "d1={d1}, d2={d2}, input1={input1:?}, input2={input2:?}");
    }

    #[rstest]
    #[case::empty_corpus("", 0.0)]
    #[case::base_and_shifted_home_row_presses("aAqQ", 0.5)]
    #[case::all_presses_on_home_row("asdfjkl;ASDFJKL:", 1.0)]
    fn home_row_usage_standard_us_cases(#[case] input: &str, #[case] expected: f64) {
        let preset = qwerty_us();
        let keyboard = preset.keyboard();
        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input).unwrap());

        let actual = cost.home_row_usage(&keyboard);

        assert!((actual - expected).abs() < 1e-12,);
    }

    #[test]
    fn home_row_usage_uses_current_layout_after_swap() {
        let preset = qwerty_us();
        let mut keyboard = preset.keyboard();

        let a_key = keyboard.layout.key_of(b'a').unwrap();
        let q_key = keyboard.layout.key_of(b'q').unwrap();
        keyboard.layout.swap(a_key, q_key);

        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text("aA").unwrap());

        assert_eq!(cost.home_row_usage(&keyboard), 0.0);
    }

    #[rstest]
    #[case::empty_corpus("", 0.0)]
    #[case::repeated_same_key("aa", 1.0)]
    #[case::shifted_and_unshifted_same_key("aA", 1.0)]
    #[case::same_finger_different_keys("1qaz", 1.0)]
    #[case::same_finger_different_hands("1=q]a'z/", 0.0)]
    #[case::different_fingers("af", 0.0)]
    #[case::mixed_bigrams("aqs", 0.5)]
    fn same_finger_bigrams_standard_us_cases(#[case] input: &str, #[case] expected: f64) {
        let preset = qwerty_us();
        let keyboard = preset.keyboard();
        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input).unwrap());

        let actual = cost.same_finger_bigrams(&keyboard);

        assert!(
            (actual - expected).abs() < TOL,
            "input {input:?}: expected {expected}, got {actual}"
        );
    }

    #[test]
    fn same_finger_bigrams_uses_current_layout_after_swap() {
        let preset = qwerty_us();
        let mut keyboard = preset.keyboard();

        let s_key = keyboard.layout.key_of(b's').unwrap();
        let q_key = keyboard.layout.key_of(b'q').unwrap();
        keyboard.layout.swap(s_key, q_key);

        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text("as").unwrap());

        assert_eq!(cost.same_finger_bigrams(&keyboard), 1.0);
    }

    #[rstest]
    #[case::empty_corpus("", 0.0)]
    #[case::repeated_same_key("aaa", 0.0)]
    #[case::shifted_and_unshifted_key("aA", 0.0)]
    #[case::hands_used_alternately("alalal", 1.0)]
    #[case::semi_alternation("aal", 0.5)]
    fn hand_alternation_standard_us_cases(#[case] input: &str, #[case] expected: f64) {
        let preset = qwerty_us();
        let keyboard = preset.keyboard();
        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input).unwrap());

        let actual = cost.hand_alternation(&keyboard);

        assert!(
            (actual - expected).abs() < TOL,
            "input {input:?}: expected {expected}, got {actual}"
        );
    }

    #[rstest]
    #[case::empty_corpus("", 0.0)]
    #[case::row_jump("qz", 1.0)]
    #[case::key_in_middle("qlz", 0.0)] // There is no jump - we assume that finger goes back to default placement
    fn row_jumping_standard_us_cases(#[case] input: &str, #[case] expected: f64) {
        let preset = qwerty_us();
        let keyboard = preset.keyboard();
        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input).unwrap());

        let actual = cost.row_jumping(&keyboard);

        assert!(
            (actual - expected).abs() < TOL,
            "input {input:?}: expected {expected}, got {actual}"
        );
    }

    #[rstest]
    #[case::empty_corpus("", 0.0)]
    #[case::single_char("a", 0.0)]
    #[case::same_key("aa", 0.0)]
    fn finger_distance_standard_us_cases(#[case] input: &str, #[case] expected: f64) {
        let preset = qwerty_us();
        let keyboard = preset.keyboard();
        let cost =
            WeightedCost::new(MetricWeights::default(), preset.corpus_from_text(input).unwrap());

        let actual = cost.finger_distance(&keyboard);

        assert!(
            (actual - expected).abs() < TOL,
            "input {input:?}: expected {expected}, got {actual}"
        );
    }
}
