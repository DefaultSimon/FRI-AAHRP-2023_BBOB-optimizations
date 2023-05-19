use miette::Result;

use crate::core::problem::BBOBProblem;

mod individual_firefly;
mod options;
pub use options::{
    get_optimized_hyperparameters,
    FireflyRunOptions,
    FullFireflyOptions,
};
use results::{
    FireflyFullOptimizationResult,
    FireflySingleRunOptimizationResult,
};
use swarm::FireflySwarm;

use crate::algorithms::common::rng::UniformU8RandomGenerator;
use crate::algorithms::common::structs::Minimum;
use crate::algorithms::firefly::utilities::PointValue;
use crate::algorithms::firefly::visualization::FireflyOptimizationMultiProgressBar;

mod results;
pub mod swarm;
pub mod utilities;
pub mod visualization;


pub enum OptimizationRunType {
    Exploration {
        run_number: usize,
        total_runs: usize,
    },
    Refinement {
        run_number: usize,
        total_runs: usize,
        best_value_before_refinement: f64,
    },
}


fn do_one_firefly_optimization_run_with_starting_point(
    problem: &mut BBOBProblem,
    options: &FireflyRunOptions,
    seed_generator: &mut UniformU8RandomGenerator,
    multi_progress_bar: &FireflyOptimizationMultiProgressBar,
    starting_point: Vec<f64>,
    run_type: OptimizationRunType,
) -> Result<FireflySingleRunOptimizationResult> {
    let problem_global_minimum = problem.name.global_minimum();

    let swarm = FireflySwarm::initialize_at_point(
        problem,
        seed_generator,
        options,
        starting_point,
    );

    do_one_firefly_optimization_run_with_swarm(
        problem_global_minimum,
        options,
        swarm,
        multi_progress_bar,
        run_type,
    )
}

fn do_one_firefly_optimization_run(
    problem: &mut BBOBProblem,
    options: &FireflyRunOptions,
    seed_generator: &mut UniformU8RandomGenerator,
    multi_progress_bar: &FireflyOptimizationMultiProgressBar,
    run_type: OptimizationRunType,
) -> Result<FireflySingleRunOptimizationResult> {
    let problem_global_minimum = problem.name.global_minimum();

    let random_swarm =
        FireflySwarm::initialize_random(problem, seed_generator, options);

    do_one_firefly_optimization_run_with_swarm(
        problem_global_minimum,
        options,
        random_swarm,
        multi_progress_bar,
        run_type,
    )
}


fn do_one_firefly_optimization_run_with_swarm(
    problem_global_minimum: f64,
    options: &FireflyRunOptions,
    mut swarm: FireflySwarm,
    multi_progress_bar: &FireflyOptimizationMultiProgressBar,
    run_type: OptimizationRunType,
) -> Result<FireflySingleRunOptimizationResult> {
    // Set up progress bar for this run.
    let progress_bar = multi_progress_bar.new_run(run_type, options)?;
    progress_bar.start();

    let mut iterations_performed: usize = 0;

    for _ in 0..options.maximum_iterations {
        iterations_performed += 1;

        // Perform a single iteration of the run.
        swarm.perform_iteration();

        // Update progress bar.
        progress_bar.update(iterations_performed, options, &swarm);

        // If stuck for `consider_stuck_after_runs` or more iterations, abort the run.
        if swarm.iterations_since_improvement
            >= options.consider_stuck_after_n_iterations
        {
            break;
        }
    }

    let best_solution = swarm
        .current_best_solution
        .expect("BUG: Invalid swarm, no solution!");

    // Clean up progress bar.
    progress_bar.finish(
        iterations_performed,
        best_solution.value,
        problem_global_minimum,
        options,
    )?;

    Ok(FireflySingleRunOptimizationResult::new(
        iterations_performed,
        best_solution,
    ))
}

pub fn run_firefly_swarm_optimization(
    mut problem: BBOBProblem,
    options: FullFireflyOptions,
) -> Result<FireflyFullOptimizationResult> {
    // Set up progress bar for this optimization run.
    let multi_progress_bar = FireflyOptimizationMultiProgressBar::new();

    // TODO We could merge the firefly algorithm with the multi-swarm optimization strategy (multiple independent swarms)
    //      See https://en.wikipedia.org/wiki/Multi-swarm_optimization

    // Parse options, initialize random generator and perform runs.
    let mut total_restarts = options.per_restart_options.len();

    let mut seed_generator =
        UniformU8RandomGenerator::new(options.random_generator_seed);

    let mut best_solution: Option<PointValue> = None;
    let mut iterations_performed_per_restart: Vec<usize> =
        Vec::with_capacity(options.per_restart_options.len());

    // Perform `restart_count` independent runs (restarts).
    for (run_index, run_options) in
        options.per_restart_options.iter().enumerate()
    {
        let run_result = do_one_firefly_optimization_run(
            &mut problem,
            run_options,
            &mut seed_generator,
            &multi_progress_bar,
            OptimizationRunType::Exploration {
                run_number: run_index + 1,
                total_runs: total_restarts,
            },
        )?;

        iterations_performed_per_restart.push(run_result.iterations_performed);

        if let Some(full_best_solution) = best_solution.as_ref() {
            if run_result.minimum.value < full_best_solution.value {
                best_solution = Some(run_result.minimum);
            }
        } else {
            best_solution = Some(run_result.minimum);
        }
    }

    let best_solution =
        best_solution.expect("Invalid firefly optimization run: no solution!");

    // If `post_process_best_options` is specified, perform refinement runs on the best solution so far,
    // updating the previous best if we found something better.
    let final_optimization_solution: Minimum =
        if let Some(post_processing_options) = options.post_process_best_options
        {
            let total_postprocessing_runs = post_processing_options.len();
            let mut best_so_far = best_solution;

            for (post_processing_run_index, post_processing_options) in
                post_processing_options.into_iter().enumerate()
            {
                let run_result =
                    do_one_firefly_optimization_run_with_starting_point(
                        &mut problem,
                        &post_processing_options,
                        &mut seed_generator,
                        &multi_progress_bar,
                        best_so_far.position.clone(),
                        OptimizationRunType::Refinement {
                            run_number: total_restarts
                                + post_processing_run_index
                                + 1,
                            total_runs: total_postprocessing_runs,
                            best_value_before_refinement: best_so_far.value,
                        },
                    )?;

                if run_result.minimum.value < best_so_far.value {
                    best_so_far = run_result.minimum;
                }
            }

            best_so_far.into()
        } else {
            best_solution.into()
        };

    // Return the final result.
    Ok(FireflyFullOptimizationResult::new(
        iterations_performed_per_restart,
        final_optimization_solution,
    ))
}
