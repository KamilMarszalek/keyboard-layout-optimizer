//! Optimization algorithms used by the keyboard layout optimizer.
//!
//! This module contains optimization routines and cost-function related types.
//! The main algorithm currently implemented is simulated annealing, which
//! searches over keyboard layout permutations by swapping pairs of keys.

pub mod cost;
pub mod sa;

use rand::{SeedableRng, rngs::SmallRng};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::keyboard::layout::Layout;
use self::sa::{AnnealingConfig, AnnealingResult, simulated_annealing};

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
