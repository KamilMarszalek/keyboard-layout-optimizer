use crate::keyboard::{geometry::Geometry, layout::Layout};

pub trait CostFunction {
    fn evaluate(&self, layout: &Layout, geometry: &Geometry) -> f64;
}
