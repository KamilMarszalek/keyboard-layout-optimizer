//! Keyboard layout optimizer core.
//!
//! This crate contains the Rust implementation of the keyboard layout optimizer.
//! It provides keyboard layout representation, physical keyboard geometry,
//! text normalization, simulated annealing and WebAssembly bindings.
//!
//! The crate is designed to be used both as a native Rust library and as a
//! WebAssembly module loaded by the TypeScript frontend.

pub mod annealing;
pub mod keyboard;
pub mod text;

#[cfg(target_arch = "wasm32")]
pub mod wasm;
