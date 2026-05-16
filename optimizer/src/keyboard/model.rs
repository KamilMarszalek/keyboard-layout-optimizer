use crate::keyboard::common::AsciiChar;

use super::geometry::Geometry;
use super::layout::Layout;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyPress {
    pub base: AsciiChar,
    pub shifted: bool,
}

/// Combines a keyboard's physical geometry with its symbol layout.
///
/// Parameter N is specified to ensure that both geometry and layout stays in sync. It also enables
/// creating smaller keyboards (especially not real ones) which is useful for writing test suits.
pub struct Keyboard<const N: usize> {
    pub geometry: Geometry<N>,
    pub layout: Layout<N>,
}

impl<const N: usize> Keyboard<N> {
    /// Builds a keyboard from a geometry and a layout with the same key count.
    pub fn new(geometry: Geometry<N>, layout: Layout<N>) -> Self {
        Self { geometry, layout }
    }
}
