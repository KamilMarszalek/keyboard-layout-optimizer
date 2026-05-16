use super::constants::US_KEY_COUNT;
use crate::keyboard::{geometry::Geometry, layout::Layout, model::Keyboard, modifier::Modifier};

impl Keyboard<US_KEY_COUNT> {
    /// Returns a keyboard using ANSI US geometry and QWERTY US symbol placement.
    pub fn qwerty_us() -> Self {
        let geometry = Geometry::standard_us();
        let modifier = Modifier::standard_us();
        let layout = Layout::qwerty_us(&modifier);
        Self::new(geometry, layout)
    }

    /// Returns a keyboard using ANSI US geometry and Dvorak US symbol placement.
    pub fn dvorak_us() -> Self {
        let geometry = Geometry::standard_us();
        let modifier = Modifier::standard_us();
        let layout = Layout::dvorak_us(&modifier);
        Self::new(geometry, layout)
    }

    /// Compatibility wrapper for the old preset-specific name.
    pub fn standard_us() -> Self {
        Self::qwerty_us()
    }
}
