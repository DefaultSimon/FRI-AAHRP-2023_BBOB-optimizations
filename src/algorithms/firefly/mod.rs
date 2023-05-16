use std::borrow::Cow;
use std::f64::consts::E;
use std::iter::zip;

use itertools::multizip;
use miette::{miette, Result};
use options::FireflyOptions;
use rng::UniformRNG;

use super::common::Minimum;
use crate::core::problem::{BBOBProblem, Bounds};

mod options;
mod rng;

// TODO Notes: we could merge the firefly algorithm with the multi-swarm optimization strategy (multiple independent swarms)
//      See https://en.wikipedia.org/wiki/Multi-swarm_optimization

pub struct PointAndValue {
    pub position: Vec<f64>,
    pub value: f64,
}

impl PointAndValue {
    pub fn new(position: Vec<f64>, value: f64) -> Self {
        Self { position, value }
    }
}

/// Individual firefly in the swarm.
#[derive(Clone)]
pub struct Firefly {
    pub position: Vec<f64>,
    pub objective_function_value: f64,
}

impl Firefly {
    pub fn new(position: Vec<f64>, problem: &mut BBOBProblem) -> Self {
        assert_eq!(
            problem.input_dimensions(),
            position.len(),
            "Position dimensions did not match!"
        );

        let objective_function_value = problem.evaluate(&position);
        Self {
            position,
            objective_function_value,
        }
    }

    pub fn move_towards(
        &mut self,
        second_firefly: &Firefly,
        problem: &mut BBOBProblem,
        zero_to_one_uniform_generator: &mut UniformRNG,
        options: &FireflyOptions,
    ) {
        // Calculate attraction coefficient (essentially how much the firefly will move towards the `other_firefly`).
        let euclidean_distance_squared = self
            .position
            .iter()
            .zip(second_firefly.position.iter())
            .map(|(first, second)| (*first - *second).powi(2))
            .sum::<f64>();

        let attraction_coefficient = options.attractiveness_coefficient
            * E.powf(
                -1f64
                    * options.light_absorption_coefficient
                    * euclidean_distance_squared,
            );

        // Add some random jitter, uniformly sampled and multiplied by the jitter coefficient, as configured.

        // Calculate the final value in each dimension.
        let final_position: Vec<f64> = zip(
            self.position.iter(),
            second_firefly.position.iter(),
        )
        .map(|(our_value, other_firefly_value)| {
            *our_value
                + attraction_coefficient * (*other_firefly_value - *our_value)
                + options.movement_jitter_coefficient
                    * (zero_to_one_uniform_generator.sample() - 0.5f64)
        })
        .collect();

        self.position = final_position;
        self.objective_function_value = problem.evaluate(&self.position);
    }
}

pub struct IterationResult {
    pub new_global_minimum: bool,
}

impl IterationResult {
    pub fn new(new_global_minimum: bool) -> Self {
        Self { new_global_minimum }
    }
}


/// Entire firefly swarm.
pub struct FireflySwarm<'problem, 'options> {
    problem: BBOBProblem<'problem>,

    best_solution: Option<PointAndValue>,

    options: &'options FireflyOptions,

    // Iterator that returns uniformly-distributed f64 in the range [0, 1].
    zero_to_one_uniform_generator: UniformRNG,

    // Vector of fireflies - this is the swarm.
    fireflies: Vec<Firefly>,
}

impl<'problem, 'options> FireflySwarm<'problem, 'options> {
    pub fn initialize(
        mut problem: BBOBProblem<'problem>,
        options: &'options FireflyOptions,
    ) -> Self {
        // Initialize the
        let input_dimensions = problem.input_dimensions();

        // Generates uniformly-distributed f64 values in the problem's range (-5 to 5).
        let mut in_bounds_uniform_generator = UniformRNG::new(
            problem.bounds(),
            options.in_bounds_random_generator_seed,
        );
        // Generates uniformly-distributed f64 values in [0, 1] range.
        let zero_to_one_uniform_generator = UniformRNG::new(
            Bounds::new(0f64, 1f64),
            options.zero_to_one_random_generator_seed,
        );

        let mut fireflies = Vec::with_capacity(options.swarm_size);
        for _ in 0..options.swarm_size {
            let position: Vec<f64> =
                in_bounds_uniform_generator.sample_multiple(input_dimensions);
            fireflies.push(Firefly::new(position, &mut problem));
        }

        Self {
            problem,
            best_solution: None,
            options,
            fireflies,
            zero_to_one_uniform_generator,
        }
    }

    fn sort_firefly_swarm_ascending(&mut self) {
        self.fireflies.sort_unstable_by(|first, second| {
            first
                .objective_function_value
                .total_cmp(&second.objective_function_value)
        });
    }

    fn is_better_than_minimum(&self, value: f64) -> bool {
        self.best_solution.is_none()
            || value < self.best_solution.as_ref().unwrap().value
    }

    fn update_minimum_value_unchecked(
        &mut self,
        value: f64,
        position: Vec<f64>,
    ) {
        self.best_solution = Some(PointAndValue::new(position, value));
    }

    pub fn perform_iteration(&mut self) -> IterationResult {
        let mut result = IterationResult::new(false);

        self.sort_firefly_swarm_ascending();

        let mut new_firefly_swarm: Vec<Firefly> =
            Vec::with_capacity(self.fireflies.len());

        for main_firefly_index in 0..self.fireflies.len() {
            let mut new_main_firefly =
                self.fireflies[main_firefly_index].clone();

            // For each firefly `F` in the swarm, compare it with each other firefly `C`.
            // If `C` is lighter (i.e. more fit, smaller objective value (we're minimizing)),
            // then `F` moves towards `C` (with some light falloff and other factors).
            // Optimization: as we'd sorted the array previously, we skip all the worse fireflies.
            for brighter_firefly in
                self.fireflies.iter().skip(main_firefly_index + 1)
            {
                if brighter_firefly.objective_function_value
                    < new_main_firefly.objective_function_value
                {
                    new_main_firefly.move_towards(
                        brighter_firefly,
                        &mut self.problem,
                        &mut self.zero_to_one_uniform_generator,
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

        self.fireflies = new_firefly_swarm;

        result
    }
}


pub fn perform_firefly_swarm_optimization(
    problem: BBOBProblem,
    options: Option<FireflyOptions>,
) -> Result<Minimum> {
    let options = options.unwrap_or_default();

    // Initialize swarm
    let mut swarm = FireflySwarm::initialize(problem, &options);
    let mut iterations_since_improvement: usize = 0;

    // Perform up to `maximum_iterations` iterations.
    for _ in 0..options.maximum_iterations {
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

    Ok(Minimum::new(
        best_solution.value,
        best_solution.position,
    ))
}
