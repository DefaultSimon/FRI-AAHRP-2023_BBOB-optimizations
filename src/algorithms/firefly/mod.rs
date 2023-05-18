use std::time::Duration;

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use miette::{IntoDiagnostic, Result};

use crate::core::problem::BBOBProblem;

mod individual_firefly;
mod options;
pub use options::{
    get_optimized_hyperparameters,
    FireflyRunOptions,
    FullFireflyOptions,
};
use swarm::FireflySwarm;

use crate::algorithms::common::rng::UniformU8RandomGenerator;
use crate::algorithms::common::structs::Minimum;
use crate::algorithms::firefly::utilities::PointValue;
use crate::algorithms::firefly::visualization::FireflyOptimizationMultiProgressBar;

pub mod swarm;
pub mod utilities;
pub mod visualization;


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
    options: FullFireflyOptions,
) -> Result<FireflyOptimizationRunResult> {
    // Set up progress bar for this optimization run.
    let multi_progress_bar = FireflyOptimizationMultiProgressBar::new();

    // TODO We could merge the firefly algorithm with the multi-swarm optimization strategy (multiple independent swarms)
    //      See https://en.wikipedia.org/wiki/Multi-swarm_optimization

    // Parse options and perform runs.
    let mut seed_generator =
        UniformU8RandomGenerator::new(options.random_generator_seed);

    let mut best_solution: Option<PointValue> = None;
    let mut iterations_performed_per_restart: Vec<usize> =
        Vec::with_capacity(options.per_restart_options.len());

    // Perform `restart_count` independent runs (restarts).
    for (run_index, run_options) in
        options.per_restart_options.iter().enumerate()
    {
        // Set up progress bar for this run.
        let progress_bar = multi_progress_bar.new_run(
            run_index + 1,
            options.per_restart_options.len(),
            run_options.maximum_iterations,
        )?;
        progress_bar.start();

        // Initialize swarm and run.
        let mut swarm = FireflySwarm::initialize(
            &mut problem,
            &mut seed_generator,
            run_options,
        );

        let mut iterations_performed: usize = 0;

        for _ in 0..run_options.maximum_iterations {
            iterations_performed += 1;

            // Perform a single iteration of the run.
            swarm.perform_iteration();

            // Update progress bar.
            progress_bar.update(iterations_performed, run_options, &swarm);

            // If stuck for `consider_stuck_after_runs` or more iterations, abort the run.
            if swarm.iterations_since_improvement
                >= run_options.consider_stuck_after_n_iterations
            {
                break;
            }
        }

        iterations_performed_per_restart.push(iterations_performed);

        let swarm_solution = swarm
            .current_best_solution
            .expect("BUG: Invalid swarm, no solution!");
        let swarm_solution_value = swarm_solution.value;

        if let Some(full_best_solution) = best_solution.as_ref() {
            if swarm_solution.value < full_best_solution.value {
                best_solution = Some(swarm_solution);
            }
        } else {
            best_solution = Some(swarm_solution);
        }


        // Clean up progress bar.
        progress_bar.finish(
            run_index + 1,
            options.per_restart_options.len(),
            iterations_performed,
            swarm_solution_value,
            problem.name.global_minimum(),
            run_options,
        )?;
    }

    let best_solution =
        best_solution.expect("Invalid optimization: no solution!");

    // Clean up multi-progress bar.
    Ok(FireflyOptimizationRunResult::new(
        iterations_performed_per_restart,
        Minimum::new(best_solution.value, best_solution.position),
    ))
}
