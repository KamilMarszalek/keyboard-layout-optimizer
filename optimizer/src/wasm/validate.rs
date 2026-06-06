use crate::{keyboard::common::AsciiChar, preset::constants::US_KEY_COUNT};

use super::dto::{AnnealingConfigDto, MetricWeightsDto};

fn ensure(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message.into()) }
}

pub(super) fn ensure_valid_annealing_config(dto: &AnnealingConfigDto) -> Result<(), String> {
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

pub(super) fn ensure_valid_metric_weights(dto: &MetricWeightsDto) -> Result<(), String> {
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

pub fn keys_to_symbols(keys: &[String]) -> Result<[AsciiChar; US_KEY_COUNT], String> {
    ensure(
        keys.len() == US_KEY_COUNT,
        format!("Expected {US_KEY_COUNT} keys but received {}", keys.len()),
    )?;

    let mut symbols = [0u8; US_KEY_COUNT];
    for (index, key) in keys.iter().enumerate() {
        let [byte] = key.as_bytes() else {
            return Err(format!(
                "Key at position {index} must be a single ASCII character, got {key:?}"
            ));
        };
        ensure(byte.is_ascii(), format!("Key at position {index} must be ASCII, got {key:?}"))?;
        symbols[index] = *byte;
    }

    Ok(symbols)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annealing::{cost::MetricWeights, sa::AnnealingConfig};

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

    fn qwerty_keys() -> Vec<String> {
        crate::preset::qwerty_us::QWERTY_US_SYMBOLS
            .iter()
            .map(|&byte| (byte as char).to_string())
            .collect()
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

    #[test]
    fn keys_to_symbols_accepts_full_qwerty_order() {
        let symbols = keys_to_symbols(&qwerty_keys()).unwrap();

        assert_eq!(symbols, crate::preset::qwerty_us::QWERTY_US_SYMBOLS);
    }

    #[test]
    fn keys_to_symbols_rejects_wrong_count() {
        let err = keys_to_symbols(&["a".to_string()]).unwrap_err();

        assert!(err.contains(&format!("Expected {US_KEY_COUNT} keys")), "got: {err}");
    }

    #[test]
    fn keys_to_symbols_rejects_multi_char_key() {
        let mut keys = qwerty_keys();
        keys[0] = "ab".to_string();

        let err = keys_to_symbols(&keys).unwrap_err();

        assert!(err.contains("position 0"), "got: {err}");
    }
}
