use crate::keyboard::{common::KEY_COUNT, model::Keyboard};

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

#[derive(Clone, Copy, Debug, Default)]
pub struct MetricBreakdown {
    pub same_finger_bigrams: f64,
    pub finger_distance: f64,
    pub home_row_usage: f64,
    pub hand_alternation: f64,
    pub row_jumping: f64,
}

impl MetricBreakdown {
    pub fn weighted_cost(&self, weigts: &MetricWeights) -> f64 {
        weigts.same_finger_bigrams * self.same_finger_bigrams + weigts.finger_distance * self.finger_distance
            - weigts.home_row_usage * self.home_row_usage
            - weigts.hand_alternation * self.hand_alternation
            + weigts.row_jumping * self.row_jumping
    }
}

pub struct Corpus {
    pub unigrams: [usize; KEY_COUNT],
    pub bigrams: [[usize; KEY_COUNT]; KEY_COUNT],
    pub total_chars: usize,
    pub total_bigrams: usize,
}

#[allow(dead_code)]
pub struct WeightedCost {
    weights: MetricWeights,
    corpus: Corpus,
}

impl WeightedCost {
    pub fn new(weights: MetricWeights, corpus: Corpus) -> Self {
        Self { weights, corpus }
    }

    pub fn evaluate<const N: usize>(&self, keyboard: &Keyboard<N>) -> f64 {
        self.evaluate_breakdown(keyboard).weighted_cost(&self.weights)
    }

    pub fn evaluate_breakdown<const N: usize>(&self, keyboard: &Keyboard<N>) -> MetricBreakdown {
        MetricBreakdown {
            same_finger_bigrams: self.same_finger_bigrams(keyboard),
            finger_distance: self.finger_distance(keyboard),
            home_row_usage: self.home_row_usage(keyboard),
            hand_alternation: self.hand_alternation(keyboard),
            row_jumping: self.row_jumping(keyboard),
        }
    }

    fn same_finger_bigrams<const N: usize>(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    fn finger_distance<const N: usize>(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    fn home_row_usage<const N: usize>(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    fn hand_alternation<const N: usize>(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
    fn row_jumping<const N: usize>(&self, _keyboard: &Keyboard<N>) -> f64 {
        todo!()
    }
}
