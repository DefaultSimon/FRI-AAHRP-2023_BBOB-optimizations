use individual_firefly::Firefly;
use miette::{miette, Result};
use super::common::rng::UniformRNG;

use super::common::structs::Minimum;
use crate::core::problem::{BBOBProblem, Bounds};

mod individual_firefly;
mod options;
pub use options::FireflyOptions;

// TODO Notes: we could merge the firefly algorithm with the multi-swarm optimization strategy (multiple independent swarms)
//      See https://en.wikipedia.org/wiki/Multi-swarm_optimization

#[derive(Clone)]
pub struct PointAndValue {
    pub position: Vec<f64>,
    pub value: f64,
}

impl PointAndValue {
    #[inline]
    pub fn new(position: Vec<f64>, value: f64) -> Self {
        Self { position, value }
    }
}

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
pub struct FireflySwarm<'problem, 'options> {
    problem: BBOBProblem<'problem>,

    minus_half_to_half_uniform_generator: UniformRNG,

    best_solution: Option<PointAndValue>,

    options: &'options FireflyOptions,

    // Vector of fireflies - this is the swarm.
    fireflies: Vec<Firefly>,
}

impl<'problem, 'options> FireflySwarm<'problem, 'options> {
    // Initialize the swarm with the given `FireflyOptions`.
    pub fn initialize(
        mut problem: BBOBProblem<'problem>,
        options: &'options FireflyOptions,
    ) -> Self {
        let input_dimensions = problem.input_dimensions;

        // Generates uniformly-distributed f64 values in the problem's range (-5 to 5).
        let mut in_bounds_uniform_generator = UniformRNG::new(
            problem.bounds(),
            options.in_bounds_random_generator_seed,
        );

        let minus_half_to_half_uniform_generator = UniformRNG::new(
            Bounds::new(-0.5f64, 0.5f64),
            options.jitter_movement_random_generator_seed,
        );

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
        self.best_solution = Some(PointAndValue::new(position, value));
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

pub struct FireflyOptimizationRunResult {
    pub iterations_performed: usize,
    pub minimum: Minimum,
}


pub fn perform_firefly_swarm_optimization(
    problem: BBOBProblem,
    options: Option<FireflyOptions>,
) -> Result<FireflyOptimizationRunResult> {
    let options = options.unwrap_or_default();

    // Initialize swarm
    let mut swarm = FireflySwarm::initialize(problem, &options);
    let mut iterations_since_improvement: usize = 0;

    // Perform up to `maximum_iterations` iterations.
    let mut iterations_performed: usize = 0;
    for iteration_index in 0..options.maximum_iterations {
        iterations_performed = iteration_index + 1;

        let result = swarm.perform_iteration();

        // Track iterations since improvement. If it reaches `stuck_run_iterations_count`,
        // we abort the run an return an early minimum so far.
        if result.new_global_minimum {
            iterations_since_improvement = 0;
        } else {
            iterations_since_improvement += 1;
        }

        if iterations_since_improvement >= options.stuck_run_iterations_count {
            break;
        }
    }

    let best_solution = swarm
        .best_solution
        .ok_or_else(|| miette!("Invalid run: no best solution at all?!"))?;

    Ok(FireflyOptimizationRunResult {
        iterations_performed,
        minimum: Minimum::new(best_solution.value, best_solution.position),
    })
}
