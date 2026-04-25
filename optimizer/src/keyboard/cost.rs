use crate::keyboard::keyboard::KeyboardGeometry;
use crate::keyboard::layout::Layout;

pub trait CostFunction {
    fn evaluate(&self, layout: &Layout, geometry: &KeyboardGeometry) -> f64;
}

pub struct MockCostFunction;

impl CostFunction for MockCostFunction {
    fn evaluate(&self, layout: &Layout, _geometry: &KeyboardGeometry) -> f64 {
        let qwerty = Layout::qwerty();
        layout
            .keys
            .iter()
            .zip(qwerty.keys.iter())
            .filter(|(a, b)| a != b)
            .count() as f64
    }
}
