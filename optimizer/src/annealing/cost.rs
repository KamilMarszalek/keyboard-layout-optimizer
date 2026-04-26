use crate::keyboard::{
    common::KEY_COUNT,
    geometry::Geometry,
    layout::{KeySymbol, Layout},
};

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

pub struct WeightedCost {
    weights: MetricWeights,
    corpus: Corpus,
}

impl WeightedCost {
    pub fn new(weights: MetricWeights, corpus: Corpus) -> Self {
        Self { weights, corpus }
    }

    pub fn evaluate(&self, layout: &Layout, geometry: &Geometry) -> f64 {
        self.evaluate_breakdown(layout, geometry).weighted_cost(&self.weights)
    }

    pub fn evaluate_breakdown(&self, layout: &Layout, geometry: &Geometry) -> MetricBreakdown {
        MetricBreakdown {
            same_finger_bigrams: self.same_finger_bigrams(layout, geometry),
            finger_distance: self.finger_distance(layout, geometry),
            home_row_usage: self.home_row_usage(layout, geometry),
            hand_alternation: self.hand_alternation(layout, geometry),
            row_jumping: self.row_jumping(layout, geometry),
        }
    }

    fn same_finger_bigrams(&self, layout: &Layout, geometry: &Geometry) -> f64 {
        todo!()
    }
    fn finger_distance(&self, layout: &Layout, geometry: &Geometry) -> f64 {
        todo!()
    }
    fn home_row_usage(&self, layout: &Layout, geometry: &Geometry) -> f64 {
        todo!()
    }
    fn hand_alternation(&self, layout: &Layout, geometry: &Geometry) -> f64 {
        todo!()
    }
    fn row_jumping(&self, layout: &Layout, geometry: &Geometry) -> f64 {
        todo!()
    }
}
