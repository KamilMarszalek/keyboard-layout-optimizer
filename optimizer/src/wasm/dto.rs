use serde::{Deserialize, Serialize};

use crate::annealing::{
    cost::{MetricBreakdown, MetricWeights},
    sa::AnnealingConfig,
};

use super::validate::{ensure_valid_annealing_config, ensure_valid_metric_weights};

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateRequestDto {
    pub text: String,
    pub weights: MetricWeightsDto,
    pub keys: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResultDto {
    pub metrics: MetricBreakdownDto,
    pub total_cost: f64,
}

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
