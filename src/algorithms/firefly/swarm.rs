use crate::algorithms::common::rng::{
    UniformF64BoundedRandomGenerator,
    UniformU8RandomGenerator,
};
use crate::algorithms::firefly::individual_firefly::Firefly;
use crate::algorithms::firefly::utilities::PointValue;
use crate::algorithms::firefly::FireflyRunOptions;
use crate::core::problem::{BBOBProblem, Bounds};

/// Entire firefly swarm.
pub struct FireflySwarm<'p: 'pref, 'pref, 'options> {
    problem: &'pref mut BBOBProblem<'p>,

    minus_half_to_half_uniform_generator: UniformF64BoundedRandomGenerator,

    options: &'options FireflyRunOptions,

    /// Vector of fireflies - this is the swarm.
    fireflies: Vec<Firefly>,

    /// Current best solution from all iterations up to this point.
    pub current_best_solution: Option<PointValue>,

    pub current_movement_jitter_coefficient: f64,

    pub iterations_since_improvement: usize,
}

impl<'p: 'pref, 'pref, 'options> FireflySwarm<'p, 'pref, 'options> {
    // Initialize the swarm with the given `FireflyOptions`.
    pub fn initialize_random(
        mut problem: &'pref mut BBOBProblem<'p>,
        seed_generator: &mut UniformU8RandomGenerator,
        options: &'options FireflyRunOptions,
    ) -> Self {
        let input_dimensions = problem.input_dimensions;

        // Generate seeds and RNGs for in-bounds and -5-to-5 random generators (using the main seed).
        let mut in_bounds_uniform_generator =
            UniformF64BoundedRandomGenerator::new(
                problem.bounds(),
                seed_generator.sample_multiple::<16>(),
            );

        let minus_half_to_half_uniform_generator =
            UniformF64BoundedRandomGenerator::new(
                Bounds::new(-0.5f64, 0.5f64),
                seed_generator.sample_multiple::<16>(),
            );

        // Generate initial population
        let mut fireflies: Vec<Firefly> = (0..options.swarm_size)
            .map(|_| {
                let initial_position: Vec<f64> = in_bounds_uniform_generator
                    .sample_multiple(input_dimensions);

                Firefly::new(initial_position, &mut problem)
            })
            .collect();

        fireflies.sort_unstable_by(|first, second| {
            second
                .objective_function_value
                .total_cmp(&first.objective_function_value)
        });

        Self {
            problem,
            minus_half_to_half_uniform_generator,
            current_best_solution: None,
            options,
            fireflies,
            current_movement_jitter_coefficient: options
                .movement_jitter_starting_coefficient,
            iterations_since_improvement: 0,
        }
    }

    pub fn initialize_at_point(
        mut problem: &'pref mut BBOBProblem<'p>,
        seed_generator: &mut UniformU8RandomGenerator,
        options: &'options FireflyRunOptions,
        initial_point: Vec<f64>,
    ) -> Self {
        let minus_half_to_half_uniform_generator =
            UniformF64BoundedRandomGenerator::new(
                Bounds::new(-0.5f64, 0.5f64),
                seed_generator.sample_multiple::<16>(),
            );

        let mut fireflies: Vec<Firefly> = (0..options.swarm_size)
            .map(|_| Firefly::new(initial_point.clone(), &mut problem))
            .collect();

        fireflies.sort_unstable_by(|first, second| {
            second
                .objective_function_value
                .total_cmp(&first.objective_function_value)
        });

        Self {
            problem,
            minus_half_to_half_uniform_generator,
            current_best_solution: None,
            options,
            fireflies,
            current_movement_jitter_coefficient: options
                .movement_jitter_starting_coefficient,
            iterations_since_improvement: 0,
        }
    }

    #[inline]
    fn is_better_than_minimum(&self, value: f64) -> bool {
        self.current_best_solution.is_none()
            || value < self.current_best_solution.as_ref().unwrap().value
    }

    #[inline]
    fn update_minimum_value_unchecked(
        &mut self,
        value: f64,
        position: Vec<f64>,
    ) {
        self.current_best_solution = Some(PointValue::new(position, value));
    }

    pub fn perform_iteration(&mut self) {
        assert_eq!(self.fireflies.len(), self.options.swarm_size);

        // Whether a better (smaller) value than the current best has been found in this iteration.
        let mut has_found_better = false;

        let mut new_firefly_swarm: Vec<Firefly> =
            Vec::with_capacity(self.fireflies.len());

        // For each firefly `new_main_firefly` in the swarm, compare it with each other firefly `brighter_firefly`.
        // If `brighter_firefly` is brighter (i.e. more fit, smaller objective value (we're minimizing)),
        // then `new_main_firefly` moves towards `brighter_firefly` (with some light falloff and other factors).

        // Optimization: as we'd sorted the array previously, we skip all the worse fireflies.

        for main_firefly_index in 0..self.fireflies.len() {
            let mut new_main_firefly =
                self.fireflies[main_firefly_index].clone();

            for brighter_firefly in
                self.fireflies.iter().skip(main_firefly_index + 1)
            {
                // The main firefly still moves, so all the fireflies that were brighter at the start
                // of the iteration might not always be brighter than the moving (main) firefly.
                if brighter_firefly.objective_function_value
                    < new_main_firefly.objective_function_value
                {
                    new_main_firefly.move_towards(
                        brighter_firefly,
                        self.problem,
                        &mut self.minus_half_to_half_uniform_generator,
                        self.current_movement_jitter_coefficient,
                        self.options,
                    );
                }
            }

            // Update minimum value if improved.
            if self.is_better_than_minimum(
                new_main_firefly.objective_function_value,
            ) {
                self.update_minimum_value_unchecked(
                    new_main_firefly.objective_function_value,
                    new_main_firefly.position.clone(),
                );

                has_found_better = true;
            }

            new_firefly_swarm.push(new_main_firefly);
        }

        // Re-sort the swarm and update self.fireflies in preparation of the next iteration.
        assert_eq!(new_firefly_swarm.len(), self.options.swarm_size);
        new_firefly_swarm.sort_unstable_by(|first, second| {
            second
                .objective_function_value
                .total_cmp(&first.objective_function_value)
        });

        self.fireflies = new_firefly_swarm;

        if has_found_better {
            self.iterations_since_improvement = 0;
        } else {
            self.iterations_since_improvement += 1;
        }

        // Update the jitter coefficient by multiplying it by the cooling factor if
        // the result is still or has recently improved.
        // Otherwise, heat it back up by the reheating factor.
        if self.iterations_since_improvement
            < self.options.movement_jitter_min_stuck_runs_to_reheat
        {
            self.current_movement_jitter_coefficient = (self
                .current_movement_jitter_coefficient
                * self.options.movement_jitter_cooling_factor)
                .max(self.options.movement_jitter_minimum_coefficient);
        } else {
            self.current_movement_jitter_coefficient = (self
                .current_movement_jitter_coefficient
                * self.options.movement_jitter_heating_factor)
                .min(self.options.movement_jitter_maximum_coefficient);
        }
    }
}
