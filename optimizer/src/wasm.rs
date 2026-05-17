use rand::SeedableRng;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    annealing::{
        cost::{MetricBreakdown, MetricWeights, WeightedCost},
        sa::{AnnealingConfig, simulated_annealing},
    },
    keyboard::{layout::Layout, model::Keyboard},
    preset::{
        constants::{US_KEY_COUNT, US_PRESS_COUNT},
        dvorak_us::dvorak_us,
        qwerty_us::qwerty_us,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeRequestDto {
    pub text: String,
    pub weights: MetricWeightsDto,
    pub annealing: AnnealingConfigDto,
    pub seed: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricWeightsDto {
    pub same_finger_bigrams: f64,
    pub finger_distance: f64,
    pub home_row_usage: f64,
    pub hand_alternation: f64,
    pub row_jumping: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnealingConfigDto {
    pub t_start: f64,
    pub t_min: f64,
    pub alpha: f64,
    pub iterations_per_temp: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeResultDto {
    pub best_layout: Vec<String>,
    pub best_cost: f64,
    pub cost_history: Vec<f64>,
    pub metrics: MetricBreakdownDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricBreakdownDto {
    pub same_finger_bigrams: f64,
    pub finger_distance: f64,
    pub home_row_usage: f64,
    pub hand_alternation: f64,
    pub row_jumping: f64,
}

impl From<MetricWeightsDto> for MetricWeights {
    fn from(dto: MetricWeightsDto) -> Self {
        Self {
            same_finger_bigrams: dto.same_finger_bigrams,
            finger_distance: dto.finger_distance,
            home_row_usage: dto.home_row_usage,
            hand_alternation: dto.hand_alternation,
            row_jumping: dto.row_jumping,
        }
    }
}

impl From<AnnealingConfigDto> for AnnealingConfig {
    fn from(dto: AnnealingConfigDto) -> Self {
        Self {
            t_start: dto.t_start,
            t_min: dto.t_min,
            alpha: dto.alpha,
            iterations_per_temp: dto.iterations_per_temp,
        }
    }
}

impl From<MetricBreakdown> for MetricBreakdownDto {
    fn from(metric: MetricBreakdown) -> Self {
        Self {
            same_finger_bigrams: metric.same_finger_bigrams,
            finger_distance: metric.finger_distance,
            home_row_usage: metric.home_row_usage,
            hand_alternation: metric.hand_alternation,
            row_jumping: metric.row_jumping,
        }
    }
}

#[wasm_bindgen]
pub fn optimize_layout(input: JsValue) -> Result<JsValue, JsValue> {
    let request: OptimizeRequestDto = serde_wasm_bindgen::from_value(input)
        .map_err(|err| JsValue::from_str(&format!("Invalid request: {err}")))?;

    let result = optimize_layout_inner(request).map_err(|err| JsValue::from_str(&err))?;

    serde_wasm_bindgen::to_value(&result)
        .map_err(|err| JsValue::from_str(&format!("Serialization failed: {err}")))
}

fn optimize_layout_inner(request: OptimizeRequestDto) -> Result<OptimizeResultDto, String> {
    let preset = qwerty_us();
    let corpus = preset
        .corpus_from_text(&request.text)
        .map_err(|err| format!("Failed to build corpus: {err:?}"))?;

    let weights: MetricWeights = request.weights.into();
    let config: AnnealingConfig = request.annealing.into();

    let cost = WeightedCost::<US_KEY_COUNT, US_PRESS_COUNT>::new(weights, corpus);

    let geometry = preset.geometry.clone();

    let cost_func = |layout: &Layout<US_KEY_COUNT>| {
        let keyboard = Keyboard::new(geometry.clone(), layout.clone());
        cost.evaluate(&keyboard)
    };

    let seed = request.seed.unwrap_or(42) as u64;

    let initial_layouts = [preset.layout, dvorak_us().layout];

    let result = initial_layouts
        .par_iter()
        .enumerate()
        .map(|(index, initial_layout)| {
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed + index as u64);
            simulated_annealing(initial_layout.clone(), &config, &mut rng, &cost_func)
        })
        .min_by(|a, b| a.best_cost.total_cmp(&b.best_cost))
        .ok_or_else(|| "No layouts available for optimization".to_string())?;

    let best_keyboard = Keyboard::new(preset.geometry.clone(), result.best_layout.clone());
    let metrics = cost.evaluate_breakdown(&best_keyboard);

    Ok(OptimizeResultDto {
        best_layout: layout_to_dto(&result.best_layout),
        best_cost: result.best_cost,
        cost_history: result.cost_history,
        metrics: metrics.into(),
    })
}

fn layout_to_dto(layout: &Layout<US_KEY_COUNT>) -> Vec<String> {
    layout.mappings_iter().map(|symbol| (symbol.base as char).to_string()).collect()
}
