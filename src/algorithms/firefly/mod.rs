use std::time::Duration;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use miette::{IntoDiagnostic, Result};

use crate::core::problem::BBOBProblem;

mod individual_firefly;
mod options;
pub use options::{FireflyRunOptions, FullFireflyOptions};
use swarm::FireflySwarm;

use crate::algorithms::common::rng::UniformU8RandomGenerator;
use crate::algorithms::common::structs::Minimum;
use crate::algorithms::firefly::utilities::PointValue;

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
    options: FullFireflyOptions,
) -> Result<FireflyOptimizationRunResult> {
    // Set up multi-progress bar.
    let multi_progress_bar = MultiProgress::new();
    let progress_bar_style_finished =
        ProgressStyle::with_template("{msg}").into_diagnostic()?;

    // TODO We could merge the firefly algorithm with the multi-swarm optimization strategy (multiple independent swarms)
    //      See https://en.wikipedia.org/wiki/Multi-swarm_optimization

    // Parse options and perform runs.
    let mut seed_generator =
        UniformU8RandomGenerator::new(options.random_generator_seed);

    let mut best_solution: Option<PointValue> = None;
    let mut iterations_performed_per_restart: Vec<usize> =
        Vec::with_capacity(options.restart_count);

    // Perform `restart_count` independent runs (restarts).
    for run_index in 0..options.restart_count {
        // Set up progress bar.
        let progress_bar_style_running = ProgressStyle::with_template(&format!(
            "[run {}/{}]  {{bar:40}} {{pos}}/{{len}} (ETA {{eta}}): {{msg}}",
            run_index + 1,
            options.restart_count,
        ))
        .into_diagnostic()?;

        let progress_bar = multi_progress_bar.add(
            ProgressBar::new(options.run_options.maximum_iterations as u64)
                .with_style(progress_bar_style_running.clone())
                .with_message("INF"),
        );

        progress_bar.enable_steady_tick(Duration::from_secs_f64(1f64 / 15f64));


        // Initialize swarm and run.
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

            progress_bar.set_position(iterations_performed as u64);
            progress_bar.set_message(format!(
                "{:.5}",
                swarm
                    .best_solution
                    .as_ref()
                    .expect("BUG: Invalid swarm, no solution at all.")
                    .value
            ));

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

        let swarm_solution = swarm
            .best_solution
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
        progress_bar.set_style(progress_bar_style_finished.clone());
        progress_bar.finish_with_message(format!(
            "[run {}/{}]  {}/{:04} iterations, minimum: {:.5}",
            run_index + 1,
            options.restart_count,
            iterations_performed,
            options.run_options.maximum_iterations,
            swarm_solution_value,
        ));
        // progress_bar.disable_steady_tick();
        progress_bar.tick();
    }

    let best_solution =
        best_solution.expect("Invalid optimization: no solution!");

    // Clean up multi-progress bar.
    Ok(FireflyOptimizationRunResult::new(
        iterations_performed_per_restart,
        Minimum::new(best_solution.value, best_solution.position),
    ))
}
