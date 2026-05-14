use crate::{
    keyboard::{
        geometry::{FingerAssignment, Row},
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
///
/// The individual metric implementations are currently placeholders and will
/// be completed in the next project milestone.
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

    fn finger_distance(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }

    fn home_row_usage(&self, keyboard: &Keyboard<N>) -> f64 {
        if self.corpus.total_chars == 0 {
            return 0.0;
        }

        let mut home_row_total = 0;

        for (press, count) in self.corpus.supported_presses.iter().zip(&self.corpus.unigrams) {
            let Some(key_idx) = keyboard.layout.key_of(press.base) else {
                continue;
            };

            let Some(key) = keyboard.geometry.key(key_idx) else {
                continue;
            };

            if key.row == Row::Home {
                home_row_total += count;
            }
        }

        home_row_total as f64 / self.corpus.total_chars as f64
    }

    fn same_finger_bigrams(&self, keyboard: &Keyboard<N>) -> f64 {
        let key_finger_assignments = self.key_finger_assignments(keyboard);
        self.bigrams_metric(|idx| key_finger_assignments[idx], |a, b| a == b)
    }

    fn hand_alternation(&self, keyboard: &Keyboard<N>) -> f64 {
        let key_finger_assignments = self.key_finger_assignments(keyboard);
        self.bigrams_metric(|idx| key_finger_assignments[idx], |a, b| a.hand != b.hand)
    }

    fn row_jumping(&self, keyboard: &Keyboard<N>) -> f64 {
        let key_row_assignment = self.key_row_assignments(keyboard);
        self.bigrams_metric(|idx| key_row_assignment[idx], |a, b| a.order().abs_diff(b.order()) > 1)
    }

    fn bigrams_metric<T, M, F>(&self, mapper: M, predicate: F) -> f64
    where
        M: Fn(usize) -> Option<T>,
        F: Fn(&T, &T) -> bool,
    {
        if self.corpus.total_bigrams == 0 {
            return 0.0;
        }

        let mut metric = 0;

        for (first_idx, second_chars) in self.corpus.bigrams.iter().enumerate() {
            let Some(first) = mapper(first_idx) else {
                continue;
            };

            for (second_idx, &count) in second_chars.iter().enumerate() {
                let Some(second) = mapper(second_idx) else {
                    continue;
                };

                if predicate(&first, &second) {
                    metric += count;
                }
            }
        }

        metric as f64 / self.corpus.total_bigrams as f64
    }

    fn key_finger_assignments(&self, keyboard: &Keyboard<N>) -> [Option<FingerAssignment>; P] {
        std::array::from_fn(|idx| {
            let press = self.corpus.supported_presses[idx];
            let key_idx = keyboard.layout.key_of(press.base)?;
            keyboard.geometry.key(key_idx).map(|key| key.finger_assignment)
        })
    }

    fn key_row_assignments(&self, keyboard: &Keyboard<N>) -> [Option<Row>; P] {
        std::array::from_fn(|idx| {
            let press = self.corpus.supported_presses[idx];
            let key_idx = keyboard.layout.key_of(press.base)?;
            keyboard.geometry.key(key_idx).map(|key| key.row)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TOL: f64 = 1e-12;

    #[rstest]
    #[case::empty_corpus("", 0.0)]
    #[case::base_and_shifted_home_row_presses("aAqQ", 0.5)]
    #[case::all_presses_on_home_row("asdfjkl;ASDFJKL:", 1.0)]
    fn home_row_usage_standard_us_cases(#[case] input: &str, #[case] expected: f64) {
        let keyboard = Keyboard::standard_us();
        let cost =
            WeightedCost::new(MetricWeights::default(), Corpus::from_text_standard_us(input));

        let actual = cost.home_row_usage(&keyboard);

        assert!((actual - expected).abs() < 1e-12,);
    }

    #[test]
    fn home_row_usage_uses_current_layout_after_swap() {
        let mut keyboard = Keyboard::standard_us();

        let a_key = keyboard.layout.key_of(b'a').unwrap();
        let q_key = keyboard.layout.key_of(b'q').unwrap();
        keyboard.layout.swap(a_key, q_key);

        let cost = WeightedCost::new(MetricWeights::default(), Corpus::from_text_standard_us("aA"));

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
        let keyboard = Keyboard::standard_us();
        let cost =
            WeightedCost::new(MetricWeights::default(), Corpus::from_text_standard_us(input));

        let actual = cost.same_finger_bigrams(&keyboard);

        assert!(
            (actual - expected).abs() < TOL,
            "input {input:?}: expected {expected}, got {actual}"
        );
    }

    #[test]
    fn same_finger_bigrams_uses_current_layout_after_swap() {
        let mut keyboard = Keyboard::standard_us();

        let s_key = keyboard.layout.key_of(b's').unwrap();
        let q_key = keyboard.layout.key_of(b'q').unwrap();
        keyboard.layout.swap(s_key, q_key);

        let cost = WeightedCost::new(MetricWeights::default(), Corpus::from_text_standard_us("as"));

        assert_eq!(cost.same_finger_bigrams(&keyboard), 1.0);
    }

    #[rstest]
    #[case::empty_corpus("", 0.0)]
    #[case::repeated_same_key("aaa", 0.0)]
    #[case::shifted_and_unshifted_key("aA", 0.0)]
    #[case::hands_used_alternately("alalal", 1.0)]
    #[case::semi_alternation("aal", 0.5)]
    fn hand_alternation_standard_us_cases(#[case] input: &str, #[case] expected: f64) {
        let keyboard = Keyboard::standard_us();
        let cost =
            WeightedCost::new(MetricWeights::default(), Corpus::from_text_standard_us(input));

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
        let keyboard = Keyboard::standard_us();
        let cost =
            WeightedCost::new(MetricWeights::default(), Corpus::from_text_standard_us(input));

        let actual = cost.row_jumping(&keyboard);

        assert!(
            (actual - expected).abs() < TOL,
            "input {input:?}: expected {expected}, got {actual}"
        );
    }
}
