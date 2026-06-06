//! Serializable data-transfer objects exchanged with the JavaScript frontend.
//!
//! `*RequestDto` types are deserialized from incoming requests and validated
//! when converted into their domain counterparts; `*ResultDto`/`*Dto` types are
//! serialized back to JS. All fields use `camelCase` to match JS conventions.

use serde::{Deserialize, Serialize};

use crate::annealing::{
    cost::{MetricBreakdown, MetricWeights},
    sa::AnnealingConfig,
};

use super::validate::{ensure_valid_annealing_config, ensure_valid_metric_weights};

/// A single key's base (unshifted) and shifted symbols.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyMappingDto {
    pub base: String,
    pub shifted: String,
}

/// A full keyboard layout as an ordered list of key mappings.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutDto {
    pub mappings: Vec<KeyMappingDto>,
}

/// Request to optimize a layout for the given `text`, metric `weights`, and
/// annealing schedule. An optional `seed` makes the run deterministic.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeRequestDto {
    pub text: String,
    pub weights: MetricWeightsDto,
    pub annealing: AnnealingConfigDto,
    pub seed: Option<u32>,
}

/// Relative weights for each typing-effort metric, validated when converted
/// into [`MetricWeights`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricWeightsDto {
    pub same_finger_bigrams: f64,
    pub finger_distance: f64,
    pub home_row_usage: f64,
    pub hand_alternation: f64,
    pub row_jumping: f64,
}

/// Simulated-annealing schedule parameters, validated when converted into
/// [`AnnealingConfig`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnealingConfigDto {
    pub t_start: f64,
    pub t_min: f64,
    pub alpha: f64,
    pub iterations_per_temp: usize,
}

/// Result of an optimization run: the best layout found, its cost, the cost
/// recorded at each step, and the breakdown of the best layout's metrics.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeResultDto {
    pub best_layout: LayoutDto,
    pub best_cost: f64,
    pub cost_history: Vec<f64>,
    pub metrics: MetricBreakdownDto,
}

/// Per-metric cost values for a single layout, before weighting.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricBreakdownDto {
    pub same_finger_bigrams: f64,
    pub finger_distance: f64,
    pub home_row_usage: f64,
    pub hand_alternation: f64,
    pub row_jumping: f64,
}

/// Request to evaluate a specific layout (given by its ordered `keys`) against
/// `text` using the supplied metric `weights`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateRequestDto {
    pub text: String,
    pub weights: MetricWeightsDto,
    pub keys: Vec<String>,
}

/// Result of evaluating a layout: the per-metric breakdown and the total
/// weighted cost.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResultDto {
    pub metrics: MetricBreakdownDto,
    pub total_cost: f64,
}

/// Relative frequency (in `[0, 1]`) of a single base symbol within a corpus.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharFrequencyDto {
    pub key: String,
    pub frequency: f64,
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
