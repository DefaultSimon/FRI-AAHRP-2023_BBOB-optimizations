use miette::Result;

use super::common::Minimum;
use crate::core::problem::BBOBProblem;

mod individual_firefly;
mod options;
pub use options::{FullFireflyOptions, RunFireflyOptions};
use swarm::FireflySwarm;

use crate::algorithms::firefly::rng::UniformU8RandomGenerator;
use crate::algorithms::firefly::utilities::PointValue;

mod rng;
mod swarm;
pub mod utilities;


pub struct FireflyOptimizationRunResult {
    // Iterations performed per-restart.
    pub iterations_performed_per_restart: Vec<usize>,

    // Minimum found throughout all restarts.
    pub minimum: Minimum,
}

impl FireflyOptimizationRunResult {
    #[inline]
    pub fn new(
        iterations_performed_per_restart: Vec<usize>,
        minimum: Minimum,
    ) -> Self {
        Self {
            iterations_performed_per_restart,
            minimum,
        }
    }
}


pub fn perform_firefly_swarm_optimization(
    mut problem: BBOBProblem,
    options: Option<FullFireflyOptions>,
) -> Result<FireflyOptimizationRunResult> {
    // TODO We could merge the firefly algorithm with the multi-swarm optimization strategy (multiple independent swarms)
    //      See https://en.wikipedia.org/wiki/Multi-swarm_optimization

    let options = options.unwrap_or_default();

    let mut seed_generator =
        UniformU8RandomGenerator::new(options.random_generator_seed);

    let mut best_solution: Option<PointValue> = None;
    let mut iterations_performed_per_restart: Vec<usize> =
        Vec::with_capacity(options.restart_count);

    // Perform `restart_count` independent runs (restarts).
    for _ in 0..options.restart_count {
        let mut swarm = FireflySwarm::initialize(
            &mut problem,
            &mut seed_generator,
            &options.run_options,
        );

        let mut iterations_since_improvement: usize = 0;
        let mut iterations_performed: usize = 0;

        for _ in 0..options.run_options.maximum_iterations {
            iterations_performed += 1;

            // Perform a single iteration of the run.
            let iteration_result = swarm.perform_iteration();

            // If reached stuck for `consider_stuck_after_runs`, abort the run.
            if iteration_result.new_global_minimum {
                iterations_since_improvement = 0;
            } else {
                iterations_since_improvement += 1;
            }

            if iterations_since_improvement
                >= options.run_options.consider_stuck_after_runs
            {
                break;
            }
        }

        iterations_performed_per_restart.push(iterations_performed);

        if let Some(swarm_solution) = swarm.best_solution {
            if let Some(full_best_solution) = best_solution.as_ref() {
                if swarm_solution.value < full_best_solution.value {
                    best_solution = Some(swarm_solution);
                }
            } else {
                best_solution = Some(swarm_solution);
            }
        }
    }

    let best_solution =
        best_solution.expect("Invalid optimization: no solution!");

    Ok(FireflyOptimizationRunResult::new(
        iterations_performed_per_restart,
        Minimum::new(best_solution.value, best_solution.position),
    ))
}
