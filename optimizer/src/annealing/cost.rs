use crate::{annealing::corpus::Corpus, keyboard::model::Keyboard};

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
    pub fn weighted_cost(&self, weigts: &MetricWeights) -> f64 {
        weigts.same_finger_bigrams * self.same_finger_bigrams
            + weigts.finger_distance * self.finger_distance
            - weigts.home_row_usage * self.home_row_usage
            - weigts.hand_alternation * self.hand_alternation
            + weigts.row_jumping * self.row_jumping
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
pub struct WeightedCost<const N: usize> {
    weights: MetricWeights,
    corpus: Corpus<N>,
}

impl<const N: usize> WeightedCost<N> {
    /// Creates a new weighted cost function from metric weights and corpus statistics.
    pub fn new(weights: MetricWeights, corpus: Corpus<N>) -> Self {
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
            same_finger_bigrams: self.same_finger_bigrams(keyboard),
            finger_distance: self.finger_distance(keyboard),
            home_row_usage: self.home_row_usage(keyboard),
            hand_alternation: self.hand_alternation(keyboard),
            row_jumping: self.row_jumping(keyboard),
        }
    }

    /// Computes the same-finger bigrams metric.
    fn same_finger_bigrams(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    /// Computes the finger distance metric.
    fn finger_distance(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    /// Computes the home-row usage metric.
    fn home_row_usage(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    /// Computes the hand alternation metric.
    fn hand_alternation(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    /// Computes the row jumping metric.
    fn row_jumping(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
}
