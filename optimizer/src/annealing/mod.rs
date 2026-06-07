//! Optimization algorithms used by the keyboard layout optimizer.
//!
//! This module contains optimization routines and cost-function related types.
//! The main algorithm currently implemented is simulated annealing, which
//! searches over keyboard layout permutations by swapping pairs of keys.

pub mod cost;
pub mod sa;

use rand::{SeedableRng, rngs::SmallRng};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use self::sa::{AnnealingConfig, AnnealingResult, simulated_annealing};
use crate::keyboard::layout::Layout;

/// Number of parallel SA runs launched by `optimize_layout_inner`.
/// The Rayon thread-pool cap in `frontend/src/wasm/engine.ts`
/// must be kept in sync with this value.
pub const WORKER_COUNT: usize = 8;

/// Runs simulated annealing from each of the provided starting layouts in
/// parallel and returns the result with the lowest cost.
///
/// Each run receives a deterministic RNG seeded with `base_seed + run_index`,
/// so the overall search is reproducible for a given set of inputs. Returns
/// `None` only if `initial_layouts` is empty.
pub fn run_multi_start<const N: usize, F>(
    initial_layouts: &[Layout<N>],
    config: &AnnealingConfig,
    base_seed: u64,
    cost_func: F,
) -> Option<AnnealingResult<N>>
where
    F: Fn(&Layout<N>) -> f64 + Sync,
{
    initial_layouts
        .par_iter()
        .enumerate()
        .map(|(index, initial_layout)| {
            let mut rng = SmallRng::seed_from_u64(base_seed + index as u64);
            simulated_annealing(initial_layout.clone(), config, &mut rng, &cost_func)
        })
        .min_by(|a, b| a.best_cost.total_cmp(&b.best_cost))
}

#[cfg(test)]
mod tests {
    use super::sa::AnnealingConfig;
    use super::*;
    use crate::keyboard::layout::Layout;
    use crate::keyboard::modifier::Modifier;
    use crate::preset::constants::US_KEY_COUNT;
    use rand::{SeedableRng, rngs::SmallRng};

    fn qwerty_layout() -> Layout<US_KEY_COUNT> {
        Layout::qwerty_us(&Modifier::standard_us())
    }

    fn fast_config() -> AnnealingConfig {
        AnnealingConfig { t_start: 1.0, t_min: 0.9, alpha: 0.9, iterations_per_temp: 1 }
    }

    #[test]
    fn run_multi_start_returns_none_on_empty_slice() {
        let result = run_multi_start::<US_KEY_COUNT, _>(&[], &fast_config(), 0, |_| 1.0);
        assert!(result.is_none());
    }

    #[test]
    fn run_multi_start_returns_best_across_runs() {
        let layouts = [qwerty_layout(), Layout::dvorak_us(&Modifier::standard_us())];
        let best = layouts[0].clone();

        let result =
            run_multi_start(
                &layouts,
                &fast_config(),
                0,
                move |layout| if *layout == best { 1.0 } else { 2.0 },
            )
            .unwrap();

        assert_eq!(result.best_cost, 1.0);
        assert_eq!(result.best_layout, layouts[0]);
    }

    #[test]
    fn run_multi_start_is_reproducible() {
        let modifier = Modifier::standard_us();
        let mut rng = SmallRng::seed_from_u64(99);
        let layouts = [Layout::random(&modifier, &mut rng), Layout::random(&modifier, &mut rng)];
        let config =
            AnnealingConfig { t_start: 1.0, t_min: 0.01, alpha: 0.5, iterations_per_temp: 3 };
        let qwerty = qwerty_layout();
        let cost = |layout: &Layout<US_KEY_COUNT>| {
            layout
                .mappings_iter()
                .zip(qwerty.mappings_iter())
                .filter(|(a, b)| a.base != b.base)
                .count() as f64
        };

        let result_a = run_multi_start(&layouts, &config, 99, cost).unwrap();
        let result_b = run_multi_start(&layouts, &config, 99, cost).unwrap();

        assert_eq!(result_a.best_cost, result_b.best_cost);
        assert_eq!(result_a.best_layout, result_b.best_layout);
    }
}
