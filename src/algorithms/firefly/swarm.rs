use crate::algorithms::firefly::individual_firefly::Firefly;
use crate::algorithms::firefly::rng::{
    UniformF64BoundedRandomGenerator,
    UniformU8RandomGenerator,
};
use crate::algorithms::firefly::utilities::PointValue;
use crate::algorithms::firefly::FireflyRunOptions;
use crate::core::problem::{BBOBProblem, Bounds};

pub struct IterationResult {
    pub new_global_minimum: bool,
}

impl IterationResult {
    #[inline]
    pub fn new(new_global_minimum: bool) -> Self {
        Self { new_global_minimum }
    }
}


/// Entire firefly swarm.
pub struct FireflySwarm<'p: 'pref, 'pref, 'options> {
    problem: &'pref mut BBOBProblem<'p>,

    minus_half_to_half_uniform_generator: UniformF64BoundedRandomGenerator,

    pub best_solution: Option<PointValue>,

    options: &'options FireflyRunOptions,

    // Vector of fireflies - this is the swarm.
    fireflies: Vec<Firefly>,
}

impl<'p: 'pref, 'pref, 'options> FireflySwarm<'p, 'pref, 'options> {
    // Initialize the swarm with the given `FireflyOptions`.
    pub fn initialize(
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
            best_solution: None,
            options,
            fireflies,
        }
    }

    #[inline]
    fn is_better_than_minimum(&self, value: f64) -> bool {
        self.best_solution.is_none()
            || value < self.best_solution.as_ref().unwrap().value
    }

    #[inline]
    fn update_minimum_value_unchecked(
        &mut self,
        value: f64,
        position: Vec<f64>,
    ) {
        self.best_solution = Some(PointValue::new(position, value));
    }

    pub fn perform_iteration(&mut self) -> IterationResult {
        assert_eq!(self.fireflies.len(), self.options.swarm_size);

        let mut result = IterationResult::new(false);

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
                        &mut self.problem,
                        &mut self.minus_half_to_half_uniform_generator,
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

                result.new_global_minimum = true;
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

        result
    }
}
