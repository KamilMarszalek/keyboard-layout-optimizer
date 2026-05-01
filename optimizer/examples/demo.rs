use optimizer::annealing::sa::{AnnealingConfig, simulated_annealing};
use optimizer::keyboard::layout::Layout;
use rand::SeedableRng;
use rand::rngs::SmallRng;

fn us_mismatch_cost<const N: usize>(layout: &Layout<N>) -> f64 {
    let qwerty = Layout::standard_us();
    layout.mappings_iter().zip(qwerty.mappings_iter()).filter(|(a, b)| a.base != b.base).count() as f64
}

fn main() {
    let config = AnnealingConfig { t_start: 1.0, t_min: 1e-4, alpha: 0.995, iterations_per_temp: 20 };

    let mut rng = SmallRng::seed_from_u64(1);

    let mut initial = Layout::standard_us();
    initial.swap(0, 1);
    initial.swap(2, 3);

    let initial_cost = us_mismatch_cost(&initial);

    let result = simulated_annealing(initial, &config, &mut rng, us_mismatch_cost);

    println!("Keyboard Layout Optimizer demo");
    println!("Initial cost: {initial_cost}");
    println!("Best cost: {}", result.best_cost);
    println!("Cost history length: {}", result.cost_history.len());
}
