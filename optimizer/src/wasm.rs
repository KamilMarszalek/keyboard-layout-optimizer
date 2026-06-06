use itertools::Itertools;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub use wasm_bindgen_rayon::init_thread_pool;

use crate::{
    annealing::{
        cost::{MetricBreakdown, MetricWeights, WeightedCost},
        run_multi_start,
        sa::AnnealingConfig,
    },
    keyboard::{layout::Layout, modifier::Modifier},
    preset::{
        constants::{US_KEY_COUNT, US_PRESS_COUNT},
        dvorak_us::dvorak_us,
        qwerty_us::qwerty_us,
    },
    text::pipeline::{map_normalized_text_to_key_presses, normalize_text},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyMappingDto {
    pub base: String,
    pub shifted: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDto {
    pub mappings: Vec<KeyMappingDto>,
}

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
    pub best_layout: LayoutDto,
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

impl TryFrom<MetricWeightsDto> for MetricWeights {
    type Error = String;
    fn try_from(dto: MetricWeightsDto) -> Result<Self, Self::Error> {
        ensure_valid_metric_weights(&dto)?;

        Ok(Self {
            same_finger_bigrams: dto.same_finger_bigrams,
            finger_distance: dto.finger_distance,
            home_row_usage: dto.home_row_usage,
            hand_alternation: dto.hand_alternation,
            row_jumping: dto.row_jumping,
        })
    }
}

impl TryFrom<AnnealingConfigDto> for AnnealingConfig {
    type Error = String;
    fn try_from(dto: AnnealingConfigDto) -> Result<Self, Self::Error> {
        ensure_valid_annealing_config(&dto)?;
        Ok(Self {
            t_start: dto.t_start,
            t_min: dto.t_min,
            alpha: dto.alpha,
            iterations_per_temp: dto.iterations_per_temp,
        })
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharFrequencyDto {
    pub key: String,
    pub frequency: f64,
}

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

fn optimize_layout_inner(request: OptimizeRequestDto) -> Result<OptimizeResultDto, String> {
    let preset = qwerty_us();
    let corpus = preset
        .corpus_from_text(&request.text)
        .map_err(|err| format!("Failed to build corpus: {err:?}"))?;

    let weights: MetricWeights = request.weights.try_into()?;
    let config: AnnealingConfig = request.annealing.try_into()?;

    let cost = WeightedCost::<US_KEY_COUNT, US_PRESS_COUNT>::new(weights, corpus, preset.geometry);

    let seed = request.seed.unwrap_or(42) as u64;
    let initial_layouts = [preset.layout, dvorak_us().layout];

    let result = run_multi_start(&initial_layouts, &config, seed, |layout| cost.evaluate(layout))
        .ok_or_else(|| "No layouts available for optimization".to_string())?;

    let metrics = cost.evaluate_breakdown(&result.best_layout);

    Ok(OptimizeResultDto {
        best_layout: layout_to_dto(&result.best_layout),
        best_cost: result.best_cost,
        cost_history: result.cost_history,
        metrics: metrics.into(),
    })
}

fn layout_to_dto(layout: &Layout<US_KEY_COUNT>) -> LayoutDto {
    LayoutDto {
        mappings: layout
            .mappings_iter()
            .map(|symbol| KeyMappingDto {
                base: symbol.base.to_string(),
                shifted: symbol.shifted.to_string(),
            })
            .collect(),
    }
}

fn ensure(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.into()) }
}

fn ensure_valid_annealing_config(dto: &AnnealingConfigDto) -> Result<(), String> {
    ensure(dto.t_start.is_finite(), "tStart must be a finite number")?;
    ensure(dto.t_min.is_finite(), "tMin must be a finite number")?;
    ensure(dto.alpha.is_finite(), "alpha must be a finite number")?;
    ensure(dto.t_start > 0.0, "tStart must be greater than 0")?;
    ensure(dto.t_min > 0.0, "tMin must be greater than 0")?;
    ensure(dto.t_start > dto.t_min, "tStart must be greater than tMin")?;
    ensure((0.0..1.0).contains(&dto.alpha), "alpha must be in range (0, 1)")?;
    ensure(dto.iterations_per_temp > 0, "iterationsPerTemp must be greater than 0")?;

    Ok(())
}

fn ensure_valid_metric_weights(dto: &MetricWeightsDto) -> Result<(), String> {
    ensure_valid_weight("sameFingerBigrams", dto.same_finger_bigrams)?;
    ensure_valid_weight("fingerDistance", dto.finger_distance)?;
    ensure_valid_weight("homeRowUsage", dto.home_row_usage)?;
    ensure_valid_weight("handAlternation", dto.hand_alternation)?;
    ensure_valid_weight("rowJumping", dto.row_jumping)?;

    Ok(())
}

fn ensure_valid_weight(name: &str, value: f64) -> Result<(), String> {
    ensure(value.is_finite(), format!("{name} must be a finite number"))?;
    ensure(value >= 0.0, format!("{name} must be non-negative"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_weights_dto() -> MetricWeightsDto {
        MetricWeightsDto {
            same_finger_bigrams: 1.0,
            finger_distance: 1.0,
            home_row_usage: 1.0,
            hand_alternation: 1.0,
            row_jumping: 1.0,
        }
    }

    fn valid_annealing_dto() -> AnnealingConfigDto {
        AnnealingConfigDto { t_start: 1.0, t_min: 0.001, alpha: 0.995, iterations_per_temp: 100 }
    }

    #[test]
    fn annealing_config_accepts_valid_dto() {
        let config = AnnealingConfig::try_from(valid_annealing_dto()).unwrap();

        assert_eq!(config.t_start, 1.0);
        assert_eq!(config.t_min, 0.001);
        assert_eq!(config.alpha, 0.995);
        assert_eq!(config.iterations_per_temp, 100);
    }

    #[test]
    fn annealing_config_rejects_non_finite_t_start() {
        let dto = AnnealingConfigDto { t_start: f64::NAN, ..valid_annealing_dto() };

        assert_eq!(
            AnnealingConfig::try_from(dto).err().as_deref(),
            Some("tStart must be a finite number")
        );
    }

    #[test]
    fn annealing_config_rejects_zero_t_min() {
        let dto = AnnealingConfigDto { t_min: 0.0, ..valid_annealing_dto() };

        assert_eq!(
            AnnealingConfig::try_from(dto).err().as_deref(),
            Some("tMin must be greater than 0")
        );
    }

    #[test]
    fn annealing_config_rejects_t_start_not_greater_than_t_min() {
        let dto = AnnealingConfigDto { t_start: 0.001, t_min: 0.001, ..valid_annealing_dto() };

        assert_eq!(
            AnnealingConfig::try_from(dto).err().as_deref(),
            Some("tStart must be greater than tMin")
        );
    }

    #[test]
    fn annealing_config_rejects_alpha_at_upper_bound() {
        let dto = AnnealingConfigDto { alpha: 1.0, ..valid_annealing_dto() };

        assert_eq!(
            AnnealingConfig::try_from(dto).err().as_deref(),
            Some("alpha must be in range (0, 1)")
        );
    }

    #[test]
    fn annealing_config_rejects_zero_iterations_per_temp() {
        let dto = AnnealingConfigDto { iterations_per_temp: 0, ..valid_annealing_dto() };

        assert_eq!(
            AnnealingConfig::try_from(dto).err().as_deref(),
            Some("iterationsPerTemp must be greater than 0")
        );
    }

    #[test]
    fn metric_weights_accepts_valid_dto() {
        let weights = MetricWeights::try_from(valid_weights_dto()).unwrap();

        assert_eq!(weights.same_finger_bigrams, 1.0);
        assert_eq!(weights.finger_distance, 1.0);
        assert_eq!(weights.home_row_usage, 1.0);
        assert_eq!(weights.hand_alternation, 1.0);
        assert_eq!(weights.row_jumping, 1.0);
    }

    #[test]
    fn metric_weights_rejects_negative_value() {
        let dto = MetricWeightsDto { finger_distance: -1.0, ..valid_weights_dto() };

        assert_eq!(
            MetricWeights::try_from(dto).err().as_deref(),
            Some("fingerDistance must be non-negative")
        );
    }

    #[test]
    fn metric_weights_rejects_non_finite_value() {
        let dto = MetricWeightsDto { row_jumping: f64::INFINITY, ..valid_weights_dto() };

        assert_eq!(
            MetricWeights::try_from(dto).err().as_deref(),
            Some("rowJumping must be a finite number")
        );
    }
}
