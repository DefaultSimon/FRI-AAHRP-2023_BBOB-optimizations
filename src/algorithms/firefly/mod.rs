use individual_firefly::Firefly;
use miette::{miette, Result};
use options::FireflyOptions;
use rand::distributions::Uniform;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use rayon::prelude::*;
use rng::UniformRNG;

use super::common::Minimum;
use crate::core::problem::{BBOBProblem, Bounds};

mod individual_firefly;
mod options;
mod rng;

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

    best_solution: Option<PointAndValue>,

    options: &'options FireflyOptions,

    // Vector of fireflies - this is the swarm.
    fireflies: Vec<Firefly>,
}

impl<'problem, 'options> FireflySwarm<'problem, 'options> {
    pub fn initialize(
        problem: BBOBProblem<'problem>,
        options: &'options FireflyOptions,
    ) -> Self {
        // Initialize the swarm.
        let input_dimensions = problem.input_dimensions;

        // Generates uniformly-distributed f64 values in the problem's range (-5 to 5).
        let mut in_bounds_uniform_generator = UniformRNG::new(
            problem.bounds(),
            options.in_bounds_random_generator_seed,
        );

        // Temporary reseeding RNG - generates u8 seeds for individual fireflies' RNGs.
        // This way we can preserve determinism, even when multi-threading.
        let u8_uniform_distribution = Uniform::new_inclusive(u8::MIN, u8::MAX);
        let mut firefly_seed_generator =
            Pcg64Mcg::from_seed(options.firefly_seed_generator_seed);

        let mut fireflies: Vec<Firefly> = (0..options.swarm_size)
            .map(|_| {
                let further_generation_seed: [u8; 16] = (0..16)
                    .map(|_| {
                        firefly_seed_generator.sample(u8_uniform_distribution)
                    })
                    .collect::<Vec<u8>>()
                    .try_into()
                    .expect("BUG: Iterator did not generate 16 u8?!?!");

                let initial_position: Vec<f64> = in_bounds_uniform_generator
                    .sample_multiple(input_dimensions);

                Firefly::new(
                    UniformRNG::new(
                        Bounds::new(0f64, 1f64),
                        further_generation_seed,
                    ),
                    initial_position,
                    &problem,
                )
            })
            .collect();

        fireflies.par_sort_unstable_by(|first, second| {
            first
                .objective_function_value
                .total_cmp(&second.objective_function_value)
        });

        Self {
            problem,
            best_solution: None,
            options,
            fireflies,
        }
    }

    #[inline]
    fn sort_firefly_swarm_descending(&mut self) {
        self.fireflies.par_sort_unstable_by(|first, second| {
            second
                .objective_function_value
                .total_cmp(&first.objective_function_value)
        });
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
        let mut result = IterationResult::new(false);


        let new_firefly_swarm: Vec<Firefly> = self
            .fireflies
            .par_iter()
            .enumerate()
            .map(|(index, firefly)| {
                let mut new_firefly = firefly.clone();

                // The swarm is always sorted (descending) at the end of the iteration.
                // This means we can all previous fireflies are "worse", and all later fireflies are "better".
                for brighter_firefly in self.fireflies.iter().skip(index + 1) {
                    if brighter_firefly.objective_function_value
                        < new_firefly.objective_function_value
                    {
                        new_firefly.move_towards(
                            brighter_firefly,
                            &self.problem,
                            self.options,
                        );
                    }
                }

                new_firefly
            })
            .collect();

        // Resort the swarm and update self.fireflies in preparation of the next iteration.
        self.sort_firefly_swarm_descending();
        self.fireflies = new_firefly_swarm;

        // Extract the best firefly and update the swarm minimum if lower.
        let best_firefly = self.fireflies.get(0).expect("Swarm is empty!");

        if self.is_better_than_minimum(best_firefly.objective_function_value) {
            self.update_minimum_value_unchecked(
                best_firefly.objective_function_value,
                best_firefly.position.clone(),
            );
            result.new_global_minimum = true;
        }

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
