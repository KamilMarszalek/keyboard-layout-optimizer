use rand::{Rng, RngExt};

use crate::keyboard::layout::Layout;

pub struct AnnealingConfig {
    pub t_start: f64,
    pub t_min: f64,
    pub alpha: f64,
    pub iterations_per_temp: usize,
}

impl Default for AnnealingConfig {
    fn default() -> Self {
        Self { t_start: 1.0, t_min: 1e-4, alpha: 0.9995, iterations_per_temp: 1_000 }
    }
}

pub struct AnnealingResult<const N: usize> {
    pub best_layout: Layout<N>,
    pub best_cost: f64,
    pub cost_history: Vec<f64>,
}

pub fn simulated_annealing<const N: usize, Func>(
    initial: Layout<N>,
    config: &AnnealingConfig,
    rng: &mut impl Rng,
    cost_func: Func,
) -> AnnealingResult<N>
where
    Func: Fn(&Layout<N>) -> f64,
{
    let mut current_layout = initial;
    let mut current_cost = cost_func(&current_layout);

    let mut best_layout = current_layout.clone();
    let mut best_cost = current_cost;

    let mut cost_history: Vec<f64> = vec![best_cost];

    let mut temperature = config.t_start;
    let len = current_layout.mappings.len();
    if len < 2 {
        return AnnealingResult { best_layout, best_cost, cost_history };
    }

    while temperature > config.t_min {
        for _ in 0..config.iterations_per_temp {
            let first = rng.random_range(0..len);
            let mut second = rng.random_range(0..len);

            while first == second {
                second = rng.random_range(0..len);
            }
            current_layout.swap(first, second);

            let new_cost = cost_func(&current_layout);
            let delta = new_cost - current_cost;

            if delta <= 0.0 || should_accept_worse(delta, temperature, rng) {
                current_cost = new_cost;

                if new_cost < best_cost {
                    best_layout = current_layout.clone();
                    best_cost = new_cost;
                }
            } else {
                current_layout.swap(first, second); // getting back to previous layout
            }
        }
        cost_history.push(best_cost);
        temperature *= config.alpha;
    }

    AnnealingResult { best_layout, best_cost, cost_history }
}

fn should_accept_worse(delta: f64, temperature: f64, rng: &mut impl Rng) -> bool {
    let prob = (-delta / temperature).exp();
    rng.random_range(0.0..1.0) < prob
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::common::KEY_COUNT;
    use rand::rngs::SmallRng;
    use rand::{RngExt, SeedableRng};

    type TestLayout = Layout<KEY_COUNT>;

    fn test_config() -> AnnealingConfig {
        AnnealingConfig { t_start: 1.0, t_min: 1e-2, alpha: 0.9, iterations_per_temp: 10 }
    }

    fn config(t_start: f64, t_min: f64, alpha: f64, iterations_per_temp: usize) -> AnnealingConfig {
        AnnealingConfig { t_start, t_min, alpha, iterations_per_temp }
    }

    fn run_sa<F>(initial: TestLayout, config: &AnnealingConfig, seed: u64, cost_func: F) -> AnnealingResult<KEY_COUNT>
    where
        F: Fn(&TestLayout) -> f64,
    {
        let mut rng = SmallRng::seed_from_u64(seed);
        simulated_annealing(initial, config, &mut rng, cost_func)
    }

    fn qwerty_mismatch_cost(layout: &TestLayout) -> f64 {
        let qwerty = Layout::standard_us();
        layout.mappings.iter().zip(qwerty.mappings.iter()).filter(|(a, b)| a.base != b.base).count() as f64
    }

    fn scrambled_layout(seed: u64, swaps: usize) -> TestLayout {
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut layout = Layout::standard_us();

        for _ in 0..swaps {
            let len = layout.mappings.len();
            let first = rng.random_range(0..len);
            let mut second = rng.random_range(0..len);
            while first == second {
                second = rng.random_range(0..len);
            }

            layout.swap(first, second);
        }

        layout
    }

    #[test]
    fn sa_best_cost_is_not_worse_than_initial_cost() {
        let initial = scrambled_layout(42, 10);
        let initial_cost = qwerty_mismatch_cost(&initial);
        let result = run_sa(initial, &test_config(), 7, qwerty_mismatch_cost);

        assert!(
            result.best_cost <= initial_cost,
            "best_cost={} should be <= initial_cost={}",
            result.best_cost,
            initial_cost
        );
    }

    #[test]
    fn cost_history_is_non_increasing() {
        let initial = scrambled_layout(42, 10);
        let result = run_sa(initial, &test_config(), 7, qwerty_mismatch_cost);
        let is_non_increasing = result.cost_history.windows(2).all(|w| w[0] >= w[1]);

        assert!(is_non_increasing, "cost history should be non-increasing: {:?}", result.cost_history);
    }

    #[test]
    fn same_seed_gives_same_result() {
        let config = test_config();
        let result_a = run_sa(Layout::standard_us(), &config, 7, qwerty_mismatch_cost);
        let result_b = run_sa(Layout::standard_us(), &config, 7, qwerty_mismatch_cost);

        assert_eq!(result_a.best_cost, result_b.best_cost);
        assert_eq!(result_a.best_layout.mappings, result_b.best_layout.mappings);
        assert_eq!(result_a.cost_history, result_b.cost_history);
    }

    #[test]
    fn best_cost_matches_best_layout() {
        let initial = scrambled_layout(42, 10);
        let result = run_sa(initial, &test_config(), 7, qwerty_mismatch_cost);

        assert_eq!(result.best_cost, qwerty_mismatch_cost(&result.best_layout));
    }

    #[test]
    fn cost_history_starts_with_initial_cost() {
        let initial = scrambled_layout(42, 10);
        let initial_cost = qwerty_mismatch_cost(&initial);
        let result = run_sa(initial, &test_config(), 7, qwerty_mismatch_cost);

        assert_eq!(result.cost_history.first().copied(), Some(initial_cost));
    }

    #[test]
    fn cost_history_has_expected_length() {
        let config = config(1.0, 0.25, 0.5, 1);
        let result = run_sa(Layout::standard_us(), &config, 0, qwerty_mismatch_cost);

        assert_eq!(result.cost_history.len(), 3);
    }

    #[test]
    fn improving_swap_is_kept() {
        let initial = Layout::standard_us();
        let initial_mappings = initial.mappings;
        let config = config(1.0, 0.5, 0.1, 1);
        let cost = |layout: &TestLayout| {
            if layout.mappings == initial_mappings { 1.0 } else { 0.0 }
        };
        let result = run_sa(initial, &config, 123, cost);

        assert_eq!(result.best_cost, 0.0);
        assert_ne!(result.best_layout.mappings, initial_mappings);
    }

    #[test]
    fn should_accept_equal_cost_move() {
        let mut rng = SmallRng::seed_from_u64(0);
        assert!(should_accept_worse(0.0, 1.0, &mut rng));
    }

    #[test]
    fn should_reject_very_bad_move_at_low_temperature() {
        let mut rng = SmallRng::seed_from_u64(0);
        assert!(!should_accept_worse(1.0, 1e-9, &mut rng));
    }
}
