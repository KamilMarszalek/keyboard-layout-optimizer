use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

use crate::preset::qwerty_us::qwerty_us;

mod dto;
mod handlers;
mod validate;

use dto::{EvaluateRequestDto, OptimizeRequestDto};
use handlers::{evaluate_layout_inner, get_char_freq_inner, layout_to_dto, optimize_layout_inner};

#[wasm_bindgen]
pub fn get_char_freq(input: &str) -> Result<JsValue, JsValue> {
    let freq = get_char_freq_inner(input);

    serde_wasm_bindgen::to_value(&freq)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}

#[wasm_bindgen]
pub fn optimize_layout(input: JsValue) -> Result<JsValue, JsValue> {
    let request: OptimizeRequestDto = serde_wasm_bindgen::from_value(input)
        .map_err(|err| JsValue::from_str(&format!("Invalid request: {err}")))?;

    let result = optimize_layout_inner(request).map_err(|err| JsValue::from_str(&err))?;

    serde_wasm_bindgen::to_value(&result)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}

#[wasm_bindgen]
pub fn qwerty_layout() -> Result<JsValue, JsValue> {
    let layout = layout_to_dto(&qwerty_us().layout);
    serde_wasm_bindgen::to_value(&layout)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}

#[wasm_bindgen]
pub fn evaluate_layout(input: JsValue) -> Result<JsValue, JsValue> {
    let request: EvaluateRequestDto = serde_wasm_bindgen::from_value(input)
        .map_err(|err| JsValue::from_str(&format!("Invalid request: {err}")))?;

    let result = evaluate_layout_inner(request).map_err(|err| JsValue::from_str(&err))?;

    serde_wasm_bindgen::to_value(&result)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}
