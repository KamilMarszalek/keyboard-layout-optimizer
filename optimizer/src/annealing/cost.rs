use crate::keyboard::keyboard::KeyboardGeometry;
use crate::keyboard::layout::Layout;

pub trait CostFunction {
    fn evaluate(&self, layout: &Layout, geometry: &KeyboardGeometry) -> f64;
}
