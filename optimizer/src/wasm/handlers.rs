//! Pure-Rust implementations behind the `#[wasm_bindgen]` entry points in
//! [`super`]. Keeping these free of `JsValue` makes them directly unit-testable.

use itertools::Itertools;
use rand::{SeedableRng, rngs::SmallRng};

use crate::{
    annealing::{
        WORKER_COUNT,
        cost::{MetricWeights, WeightedCost},
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

use super::dto::{
    CharFrequencyDto, EvaluateRequestDto, EvaluateResultDto, KeyMappingDto, LayoutDto,
    OptimizeRequestDto, OptimizeResultDto,
};
use super::validate::keys_to_symbols;

/// Normalizes `input`, counts how often each base symbol is pressed, and
/// returns each symbol's relative frequency. Returns an empty vector when the
/// normalized input contains no countable presses.
pub(super) fn get_char_freq_inner(input: &str) -> Vec<CharFrequencyDto> {
    let normalized = normalize_text(input);
    let mapper = Modifier::standard_us();

    let counts = map_normalized_text_to_key_presses(&normalized, &mapper)
        .flatten()
        .counts_by(|press| press.base);

    let total: usize = counts.values().sum();

    if total == 0 {
        return Vec::new();
    }

    counts
        .into_iter()
        .map(|(key, count)| CharFrequencyDto {
            key: (key as char).to_string(),
            frequency: count as f64 / total as f64,
        })
        .collect()
}

/// Builds a corpus from the request text and runs a multi-start simulated
/// annealing search seeded with the QWERTY and Dvorak presets plus random
/// layouts, returning the best layout found and its metric breakdown.
pub(super) fn optimize_layout_inner(
    request: OptimizeRequestDto,
) -> Result<OptimizeResultDto, String> {
    let preset = qwerty_us();
    let corpus = preset
        .corpus_from_text(&request.text)
        .map_err(|err| format!("Failed to build corpus: {err:?}"))?;

    let weights: MetricWeights = request.weights.try_into()?;
    let config: AnnealingConfig = request.annealing.try_into()?;
    let cost = WeightedCost::<US_KEY_COUNT, US_PRESS_COUNT>::new(weights, corpus, preset.geometry);
    let seed = u64::from(request.seed.unwrap_or(42));
    let mut rng = SmallRng::seed_from_u64(seed);
    let initial_layouts: [Layout<US_KEY_COUNT>; WORKER_COUNT] = std::array::from_fn(|i| match i {
        0 => preset.layout.clone(),
        1 => dvorak_us().layout,
        _ => Layout::random(&preset.modifier, &mut rng),
    });

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

/// Builds a layout from the request's key list and scores it against a corpus
/// derived from the request text, returning the metric breakdown and total
/// weighted cost without running optimization.
pub(super) fn evaluate_layout_inner(
    request: EvaluateRequestDto,
) -> Result<EvaluateResultDto, String> {
    let preset = qwerty_us();
    let corpus = preset
        .corpus_from_text(&request.text)
        .map_err(|err| format!("Failed to build corpus: {err:?}"))?;

    let weights: MetricWeights = request.weights.try_into()?;
    let symbols = keys_to_symbols(&request.keys)?;
    let layout = Layout::new(&symbols, &preset.modifier)?;

    let cost = WeightedCost::<US_KEY_COUNT, US_PRESS_COUNT>::new(weights, corpus, preset.geometry);
    let breakdown = cost.evaluate_breakdown(&layout);

    Ok(EvaluateResultDto {
        total_cost: breakdown.weighted_cost(&weights),
        metrics: breakdown.into(),
    })
}

/// Converts a domain [`Layout`] into its serializable [`LayoutDto`] form.
pub(super) fn layout_to_dto(layout: &Layout<US_KEY_COUNT>) -> LayoutDto {
    LayoutDto {
        mappings: layout
            .mappings_iter()
            .map(|symbol| KeyMappingDto {
                base: (symbol.base as char).to_string(),
                shifted: (symbol.shifted as char).to_string(),
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm::dto::MetricWeightsDto;

    fn valid_weights_dto() -> MetricWeightsDto {
        MetricWeightsDto {
            same_finger_bigrams: 1.0,
            finger_distance: 1.0,
            home_row_usage: 1.0,
            hand_alternation: 1.0,
            row_jumping: 1.0,
        }
    }

    fn qwerty_keys() -> Vec<String> {
        crate::preset::qwerty_us::QWERTY_US_SYMBOLS
            .iter()
            .map(|&byte| (byte as char).to_string())
            .collect()
    }

    fn valid_evaluate_request(keys: Vec<String>) -> EvaluateRequestDto {
        EvaluateRequestDto {
            text: "the quick brown fox".to_string(),
            weights: valid_weights_dto(),
            keys,
        }
    }

    #[test]
    fn evaluate_layout_inner_returns_metrics_for_valid_keys() {
        let result = evaluate_layout_inner(valid_evaluate_request(qwerty_keys())).unwrap();

        assert!(result.total_cost.is_finite());
        assert!(result.metrics.home_row_usage >= 0.0);
    }

    #[test]
    fn evaluate_layout_inner_rejects_non_permutation_keys() {
        let mut keys = qwerty_keys();
        keys[1] = keys[0].clone();

        let err = evaluate_layout_inner(valid_evaluate_request(keys)).unwrap_err();

        assert_eq!(err, "Provided symbols do not match modifier's base symbols");
    }

    #[test]
    fn get_char_freq_inner_returns_empty_for_empty_input() {
        assert!(get_char_freq_inner("").is_empty());
    }

    #[test]
    fn get_char_freq_inner_frequencies_sum_to_one() {
        let freq = get_char_freq_inner("the quick brown fox");

        assert!(!freq.is_empty());
        let total: f64 = freq.iter().map(|entry| entry.frequency).sum();
        assert!((total - 1.0).abs() < 1e-9, "frequencies summed to {total}");
    }
}
