use rand::{Rng, RngExt};

use crate::keyboard::common::KEY_COUNT;
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
    use rand::rngs::SmallRng;
    use rand::{Rng, RngExt, SeedableRng};

    impl AnnealingConfig {
        pub fn for_testing() -> Self {
            Self { t_start: 1.0, t_min: 1e-2, alpha: 0.9, iterations_per_temp: 10 }
        }
    }

    fn qwerty_mismatch_cost(layout: &Layout<KEY_COUNT>) -> f64 {
        let qwerty = Layout::standard_us();

        layout.mappings.iter().zip(qwerty.mappings.iter()).filter(|(a, b)| a.base != b.base).count() as f64
    }

    fn scrambled_layout(rng: &mut impl Rng) -> Layout<KEY_COUNT> {
        let mut layout = Layout::standard_us();

        for _ in 0..10 {
            let i = rng.random_range(0..layout.mappings.len());
            let mut j = rng.random_range(0..layout.mappings.len());

            while i == j {
                j = rng.random_range(0..layout.mappings.len());
            }

            layout.swap(i, j);
        }

        layout
    }

    #[test]
    fn sa_best_cost_is_not_worse_than_initial_cost() {
        let mut rng = SmallRng::seed_from_u64(42);

        let initial = scrambled_layout(&mut rng);
        let initial_cost = qwerty_mismatch_cost(&initial);

        let result = simulated_annealing(initial, &AnnealingConfig::for_testing(), &mut rng, qwerty_mismatch_cost);

        assert!(result.best_cost <= initial_cost);
    }

    #[test]
    fn cost_history_is_non_increasing() {
        let mut rng = SmallRng::seed_from_u64(0);

        let result =
            simulated_annealing(Layout::standard_us(), &AnnealingConfig::for_testing(), &mut rng, qwerty_mismatch_cost);

        let is_non_increasing = result.cost_history.windows(2).all(|w| w[0] >= w[1]);

        assert!(is_non_increasing);
    }

    #[test]
    fn same_seed_gives_same_result() {
        let config = AnnealingConfig::for_testing();

        let result_a =
            simulated_annealing(Layout::standard_us(), &config, &mut SmallRng::seed_from_u64(7), qwerty_mismatch_cost);

        let result_b =
            simulated_annealing(Layout::standard_us(), &config, &mut SmallRng::seed_from_u64(7), qwerty_mismatch_cost);

        assert_eq!(result_a.best_cost, result_b.best_cost);
        assert_eq!(result_a.best_layout.mappings, result_b.best_layout.mappings);
    }
}
