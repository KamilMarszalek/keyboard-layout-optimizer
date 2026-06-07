//! WebAssembly entry points exposed to the JavaScript frontend.
//!
//! Each `#[wasm_bindgen]` function deserializes a request from [`JsValue`],
//! delegates to a pure-Rust handler in `handlers`, and serializes the result
//! back to [`JsValue`]. All errors are surfaced to JS as string-valued [`JsValue`]s.

use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

use crate::preset::qwerty_us::qwerty_us;

pub mod dto;
mod handlers;
mod validate;

use dto::{EvaluateRequestDto, OptimizeRequestDto};
use handlers::{evaluate_layout_inner, get_char_freq_inner, layout_to_dto, optimize_layout_inner};

/// Computes the relative frequency of each base symbol in `input` and returns
/// it as a serialized array of `CharFrequencyDto`.
#[wasm_bindgen]
pub fn get_char_freq(input: &str) -> Result<JsValue, JsValue> {
    let freq = get_char_freq_inner(input);

    serde_wasm_bindgen::to_value(&freq)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}

/// Runs simulated-annealing layout optimization for the given request and
/// returns the best layout, its cost, the cost history, and a metric breakdown.
#[wasm_bindgen]
pub fn optimize_layout(input: JsValue) -> Result<JsValue, JsValue> {
    let request: OptimizeRequestDto = serde_wasm_bindgen::from_value(input)
        .map_err(|err| JsValue::from_str(&format!("Invalid request: {err}")))?;

    let result = optimize_layout_inner(request).map_err(|err| JsValue::from_str(&err))?;

    serde_wasm_bindgen::to_value(&result)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}

/// Returns the standard US QWERTY layout as a serialized `LayoutDto`,
/// for use as a reference layout in the frontend.
#[wasm_bindgen]
pub fn qwerty_layout() -> Result<JsValue, JsValue> {
    let layout = layout_to_dto(&qwerty_us().layout);
    serde_wasm_bindgen::to_value(&layout)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}

/// Evaluates a caller-supplied layout against the given text and weights,
/// returning the per-metric breakdown and total weighted cost without optimizing.
#[wasm_bindgen]
pub fn evaluate_layout(input: JsValue) -> Result<JsValue, JsValue> {
    let request: EvaluateRequestDto = serde_wasm_bindgen::from_value(input)
        .map_err(|err| JsValue::from_str(&format!("Invalid request: {err}")))?;

    let result = evaluate_layout_inner(request).map_err(|err| JsValue::from_str(&err))?;

    serde_wasm_bindgen::to_value(&result)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}
