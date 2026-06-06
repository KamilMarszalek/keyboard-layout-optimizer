use itertools::Itertools;
use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

use crate::{
    keyboard::modifier::Modifier,
    preset::qwerty_us::qwerty_us,
    text::pipeline::{map_normalized_text_to_key_presses, normalize_text},
};

mod dto;
mod handlers;
mod validate;

use dto::{CharFrequencyDto, EvaluateRequestDto, OptimizeRequestDto};
use handlers::{evaluate_layout_inner, layout_to_dto, optimize_layout_inner};

#[wasm_bindgen]
pub fn get_char_freq(input: &str) -> Result<JsValue, JsValue> {
    let normalized = normalize_text(input);
    let mapper = Modifier::standard_us();

    let counts = map_normalized_text_to_key_presses(&normalized, &mapper)
        .flatten()
        .counts_by(|press| press.base);

    let total: usize = counts.values().sum();

    if total == 0 {
        return serde_wasm_bindgen::to_value(&Vec::<CharFrequencyDto>::new())
            .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")));
    }

    let freq: Vec<CharFrequencyDto> = counts
        .into_iter()
        .map(|(key, count)| CharFrequencyDto {
            key: (key as char).to_string(),
            frequency: count as f64 / total as f64,
        })
        .collect();

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
