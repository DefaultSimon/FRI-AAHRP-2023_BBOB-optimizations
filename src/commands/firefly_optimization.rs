use std::num::NonZeroUsize;
use std::time::Instant;

use clap::{Args, Subcommand};
use indicatif::MultiProgress;
use itertools::Itertools;
use miette::{miette, Result};

use crate::algorithms::firefly::{
    perform_firefly_swarm_optimization,
    FireflyRunOptions,
    FullFireflyOptions,
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
        run_options: FireflyRunOptions {
            swarm_size: 80,
            maximum_iterations: 2000,
            consider_stuck_after_runs: 500,
            attractiveness_coefficient: 1f64,
            light_absorption_coefficient: 0.025,
            movement_jitter_starting_coefficient: 0.1,
            movement_jitter_minimum_coefficient: 0.005,
            movement_jitter_cooling_factor: 0.98,
        },
    };

    match problem {
        // OK (delta=0.00005)
        BBOBFunction::Sphere => FullFireflyOptions {
            random_generator_seed: [
                133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79,
                56, 2, 31,
            ],
            restart_count: 4,
            run_options: FireflyRunOptions {
                swarm_size: 80,
                maximum_iterations: 5000,
                consider_stuck_after_runs: 500,
                attractiveness_coefficient: 1f64,
                light_absorption_coefficient: 0.02,
                movement_jitter_starting_coefficient: 0.1,
                movement_jitter_minimum_coefficient: 0.005,
                movement_jitter_cooling_factor: 0.98,
            },
        },
        // NOT OK
        BBOBFunction::SeparableEllipsoidal => FullFireflyOptions {
            random_generator_seed: [
                50, 61, 220, 154, 210, 7, 26, 14, 226, 210, 241, 67, 109, 149,
                214, 27,
            ],
            restart_count: 4,
            run_options: FireflyRunOptions {
                swarm_size: 80,
                maximum_iterations: 14000,
                consider_stuck_after_runs: 500,
                attractiveness_coefficient: 0.99f64,
                light_absorption_coefficient: 0.001,
                movement_jitter_starting_coefficient: 0.3,
                movement_jitter_minimum_coefficient: 0.1,
                movement_jitter_cooling_factor: 0.9999,
            },
        },
        // NOT OK
        BBOBFunction::Rastrigin => FullFireflyOptions {
            random_generator_seed: [
                133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79,
                56, 2, 31,
            ],
            restart_count: 4,
            run_options: FireflyRunOptions {
                swarm_size: 40,
                maximum_iterations: 5000,
                consider_stuck_after_runs: 500,
                attractiveness_coefficient: 1f64,
                light_absorption_coefficient: 0.025,
                movement_jitter_starting_coefficient: 0.1,
                movement_jitter_minimum_coefficient: 0.01,
                movement_jitter_cooling_factor: 0.995,
            },
        },
        // NOT OK
        BBOBFunction::BucheRastrigin => defaults,
        // NOT OK
        BBOBFunction::LinearSlope => defaults,
        // NOT OK
        BBOBFunction::AttractiveSector => defaults,
        // NOT OK
        BBOBFunction::StepEllipsoidal => defaults,
        // NOT OK
        BBOBFunction::RosenbrockFunction => defaults,
        // NEARLY THERE (delta=5.27988)
        BBOBFunction::RosenbrockFunctionRotated => FullFireflyOptions {
            random_generator_seed: [
                131, 66, 79, 177, 132, 191, 158, 217, 16, 170, 134, 80, 79, 56,
                2, 31,
            ],
            restart_count: 4,
            run_options: FireflyRunOptions {
                swarm_size: 30,
                maximum_iterations: 100000,
                consider_stuck_after_runs: 500,
                attractiveness_coefficient: 1f64,
                light_absorption_coefficient: 0.025,
                movement_jitter_starting_coefficient: 0.1,
                movement_jitter_minimum_coefficient: 0.01,
                movement_jitter_cooling_factor: 0.995,
            },
        },
        // NOT OK
        BBOBFunction::Ellipsoidal => defaults,
        // NEARLY THERE (delta=25.36596)
        BBOBFunction::Discus => defaults,
        // NOT OK
        BBOBFunction::BentCigar => defaults,
        // NOT OK
        BBOBFunction::SharpRidge => defaults,
        // OK (delta=0.00107)
        BBOBFunction::DifferentPowers => FullFireflyOptions {
            random_generator_seed: [
                133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79,
                56, 2, 31,
            ],
            restart_count: 4,
            run_options: FireflyRunOptions {
                swarm_size: 80,
                maximum_iterations: 4000,
                consider_stuck_after_runs: 500,
                attractiveness_coefficient: 1f64,
                light_absorption_coefficient: 0.02,
                movement_jitter_starting_coefficient: 0.1,
                movement_jitter_minimum_coefficient: 0.01,
                movement_jitter_cooling_factor: 0.999,
            },
        },
        // NOT OK
        BBOBFunction::RastriginMultiModal => defaults,
        // NOT OK
        BBOBFunction::Weierstrass => defaults,
        // NOT OK
        BBOBFunction::SchafferF7 => defaults,
        // NOT OK
        BBOBFunction::SchafferF7IllConditioned => defaults,
        // NOT OK
        BBOBFunction::CompositeGriewankRosenbrockF8F2 => defaults,
        // NOT OK
        BBOBFunction::Schwefel => defaults,
        // NOT OK
        BBOBFunction::GallagherGaussian101MePeaks => defaults,
        // NOT OK
        BBOBFunction::GallagherGaussian21HiPeaks => defaults,
        // NOT OK
        BBOBFunction::Katsuura => defaults,
        // NOT OK
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

        println!(
            "[Problem {:02}/{:02}: {}]",
            bbob_function.index(),
            ALL_BBOB_FUNCTIONS.len(),
            bbob_function.name(),
        );

        let optimization_results = perform_firefly_swarm_optimization(
            problem,
            optimized_hyperparameters.clone(),
        )?;

        let problem_delta_time = problem_start_time.elapsed().as_secs_f64();


        let formatted_parameters = optimization_results
            .minimum
            .vector
            .iter()
            .map(|parameter| parameter.to_string())
            .join(",");

        println!(
            "  Optimized. Performed {:?}/{} iterations in {:.4} seconds",
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
        optimized_hyperparameters.clone(),
    )?;

    let problem_delta_time = problem_start_time.elapsed().as_secs_f64();


    let formatted_parameters = optimization_results
        .minimum
        .vector
        .iter()
        .map(|parameter| parameter.to_string())
        .join(",");

    println!();
    println!(
        "Problem {:02}/{:02}: {}  -  {:?}/{} iterations, {:.4} seconds",
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
