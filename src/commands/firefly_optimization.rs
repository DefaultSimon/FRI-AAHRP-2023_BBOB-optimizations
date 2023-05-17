use std::num::NonZeroUsize;
use std::time::Instant;

use clap::{Args, Subcommand};
use itertools::Itertools;
use miette::{miette, Result};

use crate::algorithms::firefly::{
    perform_firefly_swarm_optimization,
    FullFireflyOptions,
    RunFireflyOptions,
};
use crate::core::functions::{BBOBFunction, ALL_BBOB_FUNCTIONS};
use crate::core::suite::BBOBSuite;

#[derive(Args)]
pub struct CLIFireflyOptimizationArgs {
    #[command(subcommand)]
    pub mode: CLIFireflyOptimizationMode,
}

#[derive(Subcommand)]
pub enum CLIFireflyOptimizationMode {
    #[command(name = "all", about = "Optimize all 24 problems.")]
    AllProblems,

    #[command(name = "single", about = "Optimize a specific problem.")]
    OneProblem(CLIRunOneArgs),
}

#[derive(Args)]
pub struct CLIRunOneArgs {
    #[arg(
        short = 'p',
        long = "problem",
        help = "What problem to run (1 to 24)."
    )]
    pub problem_number: NonZeroUsize,
}


fn get_optimized_hyperparameters(problem: BBOBFunction) -> FullFireflyOptions {
    let defaults = FullFireflyOptions {
        random_generator_seed: [
            133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79, 56, 2,
            31,
        ],
        restart_count: 4,
        run_options: RunFireflyOptions {
            swarm_size: 150,
            maximum_iterations: 2000,
            consider_stuck_after_runs: 500,
            attractiveness_coefficient: 1f64,
            light_absorption_coefficient: 0.025,
            movement_jitter_coefficient: 0.01,
        },
    };

    match problem {
        BBOBFunction::Sphere => defaults,
        BBOBFunction::SeparableEllipsoidal => defaults,
        BBOBFunction::Rastrigin => defaults,
        BBOBFunction::BucheRastrigin => defaults,
        BBOBFunction::LinearSlope => defaults,
        BBOBFunction::AttractiveSector => defaults,
        BBOBFunction::StepEllipsoidal => defaults,
        BBOBFunction::RosenbrockFunction => defaults,
        BBOBFunction::RosenbrockFunctionRotated => defaults,
        BBOBFunction::Ellipsoidal => defaults,
        BBOBFunction::Discus => defaults,
        BBOBFunction::BentCigar => defaults,
        BBOBFunction::SharpRidge => defaults,
        BBOBFunction::DifferentPowers => defaults,
        BBOBFunction::RastriginMultiModal => defaults,
        BBOBFunction::Weierstrass => defaults,
        BBOBFunction::SchafferF7 => defaults,
        BBOBFunction::SchafferF7IllConditioned => defaults,
        BBOBFunction::CompositeGriewankRosenbrockF8F2 => defaults,
        BBOBFunction::Schwefel => defaults,
        BBOBFunction::GallagherGaussian101MePeaks => defaults,
        BBOBFunction::GallagherGaussian21HiPeaks => defaults,
        BBOBFunction::Katsuura => defaults,
        BBOBFunction::LunacekBiRastrigin => defaults,
    }
}

pub fn cmd_run_all_problems() -> Result<()> {
    println!("-- Running firefly optimization on all 24 problems. --");
    println!();

    // Initialize coco / BBOB suite.
    let mut suite = BBOBSuite::new()?;

    let total_start_time = Instant::now();

    // TODO We can actually parallelize this by running multiple individual problems at the same time.
    for bbob_function in ALL_BBOB_FUNCTIONS {
        let problem_start_time = Instant::now();

        let optimized_hyperparameters =
            get_optimized_hyperparameters(bbob_function);
        let problem = suite.problem(bbob_function, None)?;

        let optimization_results = perform_firefly_swarm_optimization(
            problem,
            Some(optimized_hyperparameters.clone()),
        )?;

        let problem_delta_time = problem_start_time.elapsed().as_secs_f64();


        let formatted_parameters = optimization_results
            .minimum
            .vector
            .iter()
            .map(|parameter| parameter.to_string())
            .join(",");

        println!(
            "[Problem {:02}/{:02}: {}] - {:?}/{} iterations, {:.4} seconds",
            bbob_function.index(),
            ALL_BBOB_FUNCTIONS.len(),
            bbob_function.name(),
            optimization_results.iterations_performed_per_restart,
            optimized_hyperparameters.run_options.maximum_iterations,
            problem_delta_time
        );

        println!(
            "  Minimum: {}",
            optimization_results.minimum.value,
        );
        println!("  At: [{}]", formatted_parameters);

        println!(
            "  Distance from global minimum: {:.5}",
            optimization_results.minimum.value - bbob_function.global_minimum()
        );
        println!();
    }

    let total_delta_time = total_start_time.elapsed().as_secs_f64();

    println!(
        "\n-- Optimized all 24 problems in {:.4} seconds. --",
        total_delta_time
    );

    Ok(())
}

pub fn cmd_run_specific_problem(args: CLIRunOneArgs) -> Result<()> {
    let bbob_function =
        BBOBFunction::from_function_index(args.problem_number.into())
            .ok_or_else(|| {
                miette!("Invalid problem index (not in 1-24 range).")
            })?;

    println!(
        "-- Running firefly optimization on problem {} ({}). --",
        bbob_function.index(),
        bbob_function.name()
    );
    println!();

    let problem_start_time = Instant::now();

    // Initialize coco / BBOB suite.
    let mut suite = BBOBSuite::new()?;

    let optimized_hyperparameters = get_optimized_hyperparameters(bbob_function);
    let problem = suite.problem(bbob_function, None)?;

    let optimization_results = perform_firefly_swarm_optimization(
        problem,
        Some(optimized_hyperparameters.clone()),
    )?;

    let problem_delta_time = problem_start_time.elapsed().as_secs_f64();


    let formatted_parameters = optimization_results
        .minimum
        .vector
        .iter()
        .map(|parameter| parameter.to_string())
        .join(",");

    println!(
        "[Problem {:02}/{:02}: {}] - {:?}/{} iterations, {:.4} seconds",
        bbob_function.index(),
        ALL_BBOB_FUNCTIONS.len(),
        bbob_function.name(),
        optimization_results.iterations_performed_per_restart,
        optimized_hyperparameters.run_options.maximum_iterations,
        problem_delta_time
    );

    println!(
        "  Minimum: {}",
        optimization_results.minimum.value,
    );
    println!("  At: [{}]", formatted_parameters);

    println!(
        "  Distance from global minimum: {:.5}",
        optimization_results.minimum.value - bbob_function.global_minimum()
    );
    println!();

    Ok(())
}

pub fn cmd_run_firefly_optimization(
    args: CLIFireflyOptimizationArgs,
) -> Result<()> {
    match args.mode {
        CLIFireflyOptimizationMode::AllProblems => cmd_run_all_problems(),
        CLIFireflyOptimizationMode::OneProblem(one_args) => {
            cmd_run_specific_problem(one_args)
        }
    }
}
