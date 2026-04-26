use super::common::KeyIndex;
use super::geometry::Geometry;
use super::layout::Layout;

#[allow(dead_code)]
struct KeyPress {
    letter: KeyIndex,
    shift: bool,
}

#[allow(dead_code)]
pub struct Keyboard {
    geometry: Geometry,
    layout: Layout,
}

impl Keyboard {
    pub fn new(geometry: Geometry, layout: Layout) -> Self {
        Self { geometry, layout }
    }

    pub fn standard_us() -> Self {
        let geometry = Geometry::standard_us();
        let layout = Layout::standard_us();
        Self::new(geometry, layout)
    }
}
