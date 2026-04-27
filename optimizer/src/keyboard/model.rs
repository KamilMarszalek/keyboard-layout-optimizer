use super::common::{KEY_COUNT, KeyIndex};
use super::geometry::Geometry;
use super::layout::Layout;

#[allow(dead_code)]
struct KeyPress {
    letter: KeyIndex,
    shift: bool,
}

#[allow(dead_code)]
pub struct Keyboard<const N: usize> {
    pub geometry: Geometry<N>,
    pub layout: Layout<N>,
}

impl<const N: usize> Keyboard<N> {
    pub fn new(geometry: Geometry<N>, layout: Layout<N>) -> Self {
        Self { geometry, layout }
    }
}

impl Keyboard<KEY_COUNT> {
    pub fn standard_us() -> Self {
        let geometry = Geometry::standard_us();
        let layout = Layout::standard_us();
        Self::new(geometry, layout)
    }
}
