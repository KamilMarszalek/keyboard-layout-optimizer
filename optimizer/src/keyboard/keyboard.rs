use super::common::KeyIndex;
use super::geometry::Geometry;
use super::layout::Layout;

use std::sync::Arc;

struct KeyPress {
    letter: KeyIndex,
    shift: bool,
}

pub struct Keyboard {
    geometry: Arc<Geometry>,
    layout: Layout,
}

impl Keyboard {
    pub fn new(geometry: Arc<Geometry>, layout: Layout) -> Self {
        Self { geometry, layout }
    }

    pub fn standard_us() -> Self {
        let geometry = Geometry::standard_us();
        let layout = Layout::standard_us();
        Self::new(Arc::new(geometry), layout)
    }
}
