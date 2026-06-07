//! Built-in keyboard presets.
//!
//! This module provides the `KeyboardPreset` type and ready-made presets for
//! standard layouts (US QWERTY, US Dvorak) along with shared constants and the
//! geometry implementation for a standard US keyboard.

pub mod constants;
pub mod dvorak_us;
pub mod keyboard_preset;
mod preset_impls;
pub mod qwerty_us;
