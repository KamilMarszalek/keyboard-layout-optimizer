use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn normalize_demo(input: &str) -> String {
    crate::text::normalize::normalize_text(input)
}

#[wasm_bindgen]
pub fn optimizer_demo_message() -> String {
    String::from("Keyboard optimizer WASM module loaded")
}
