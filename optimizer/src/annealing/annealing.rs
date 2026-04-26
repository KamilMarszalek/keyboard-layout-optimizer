use rand::{Rng, RngExt};

use crate::{
    annealing::cost::CostFunction,
    keyboard::{geometry::Geometry, layout::Layout},
};

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

pub struct AnnealingResult {
    pub best_layout: Layout,
    pub best_cost: f64,
    pub cost_history: Vec<f64>,
}

pub fn simulated_annealing(
    initial: Layout,
    geometry: &Geometry,
    cost_func: &dyn CostFunction,
    config: &AnnealingConfig,
    rng: &mut impl Rng,
) -> AnnealingResult {
    let mut current_layout = initial;
    let mut current_cost = cost_func.evaluate(&current_layout, geometry);

    let mut best_layout = current_layout.clone();
    let mut best_cost = current_cost;

    let mut cost_history: Vec<f64> = vec![best_cost];

    let mut temperature = config.t_start;

    while temperature > config.t_min {
        for _ in 0..config.iterations_per_temp {
            let new_layout = generate_new_layout(&current_layout, rng);
            let new_cost = cost_func.evaluate(&new_layout, geometry);
            let delta = new_cost - current_cost;

            if delta <= 0.0 || should_accept_worse(delta, temperature, rng) {
                current_layout = new_layout;
                current_cost = new_cost;

                if new_cost < best_cost {
                    best_layout = current_layout.clone();
                    best_cost = new_cost;
                }
            }
        }
        cost_history.push(best_cost);
        temperature *= config.alpha;
    }

    AnnealingResult { best_layout, best_cost, cost_history }
}

fn generate_new_layout(current_layout: &Layout, rng: &mut impl Rng) -> Layout {
    let len = current_layout.mappings.len();
    if len < 2 {
        return current_layout.clone();
    }
    let first = rng.random_range(0..len);
    let mut second = rng.random_range(0..len);
    while first == second {
        second = rng.random_range(0..len);
    }
    current_layout.swap(first, second)
}

fn should_accept_worse(delta: f64, temperature: f64, rng: &mut impl Rng) -> bool {
    let prob = (-delta / temperature).exp();
    rng.random_range(0.0..1.0) < prob
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::SmallRng;

    struct MockCost;
    impl CostFunction for MockCost {
        fn evaluate(&self, layout: &Layout, _: &Geometry) -> f64 {
            let qwerty = Layout::standard_us();
            layout.mappings.iter().zip(qwerty.mappings.iter()).filter(|(a, b)| a.base != b.base).count() as f64
        }
    }

    impl AnnealingConfig {
        pub fn for_testing() -> Self {
            Self { t_start: 1.0, t_min: 1e-2, alpha: 0.9, iterations_per_temp: 10 }
        }
    }

    #[test]
    fn sa_improves_cost() {
        let mut rng = SmallRng::seed_from_u64(42);
        let geometry = Geometry::standard_us();
        let cost_fn = MockCost;

        let mut initial = Layout::standard_us();
        for _ in 0..10 {
            let i = rng.random_range(0..26);
            let j = rng.random_range(0..26);
            initial = initial.swap(i, j);
        }

        let initial_cost = cost_fn.evaluate(&initial, &geometry);
        let result = simulated_annealing(initial, &geometry, &cost_fn, &AnnealingConfig::for_testing(), &mut rng);

        assert!(result.best_cost <= initial_cost);
    }

    #[test]
    fn cost_history_is_non_increasing() {
        let mut rng = SmallRng::seed_from_u64(0);
        let geometry = Geometry::standard_us();
        let result =
            simulated_annealing(Layout::standard_us(), &geometry, &MockCost, &AnnealingConfig::for_testing(), &mut rng);

        let is_non_increasing = result.cost_history.windows(2).all(|w| w[0] >= w[1]);
        assert!(is_non_increasing);
    }

    #[test]
    fn same_seed_gives_same_result() {
        let geometry = Geometry::standard_us();

        let result_a = simulated_annealing(
            Layout::standard_us(),
            &geometry,
            &MockCost,
            &AnnealingConfig::for_testing(),
            &mut SmallRng::seed_from_u64(7),
        );
        let result_b = simulated_annealing(
            Layout::standard_us(),
            &geometry,
            &MockCost,
            &AnnealingConfig::for_testing(),
            &mut SmallRng::seed_from_u64(7),
        );

        assert_eq!(result_a.best_cost, result_b.best_cost);
        assert_eq!(result_a.best_layout.mappings, result_b.best_layout.mappings);
    }
}
