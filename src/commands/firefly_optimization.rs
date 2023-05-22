use std::num::NonZeroUsize;
use std::time::Instant;

use clap::{Args, Subcommand};
use itertools::Itertools;
use miette::{miette, Result};

use crate::algorithms::firefly::{
    get_optimized_hyperparameters,
    run_firefly_swarm_optimization,
};
use crate::core::functions::{BBOBFunctionType, ALL_BBOB_FUNCTIONS};
use crate::core::suite::BBOBSuite;

#[derive(Args, Eq, PartialEq)]
pub struct CLIFireflyOptimizationArgs {
    #[command(subcommand)]
    pub mode: CLIFireflyOptimizationMode,
}

#[derive(Subcommand, Eq, PartialEq)]
pub enum CLIFireflyOptimizationMode {
    #[command(name = "all", about = "Optimize all 24 problems.")]
    AllProblems,

    #[command(name = "single", about = "Optimize a specific problem.")]
    OneProblem(CLIRunOneArgs),
}

#[derive(Args, Eq, PartialEq)]
pub struct CLIRunOneArgs {
    #[arg(
        short = 'p',
        long = "problem",
        help = "What problem to run (1 to 24)."
    )]
    pub problem_number: NonZeroUsize,
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
        let problem = suite.problem(bbob_function)?;

        println!(
            "[[Problem {:02}/{:02} ({}) | global minimum is {:.4}]]",
            bbob_function.index(),
            ALL_BBOB_FUNCTIONS.len(),
            bbob_function.name(),
            bbob_function.global_minimum(),
        );

        let optimization_results = run_firefly_swarm_optimization(
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
            "\n  Optimized in {:.4} seconds",
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
        BBOBFunctionType::from_function_index(args.problem_number.into())
            .ok_or_else(|| {
                miette!("Invalid problem index (not in 1-24 range).")
            })?;

    println!(
        "[[Problem {} ({}) | global minimum is {:.4}]]",
        bbob_function.index(),
        bbob_function.name(),
        bbob_function.global_minimum(),
    );

    let problem_start_time = Instant::now();

    // Initialize coco / BBOB suite.
    let mut suite = BBOBSuite::new()?;

    let optimized_hyperparameters = get_optimized_hyperparameters(bbob_function);
    let problem = suite.problem(bbob_function)?;

    let optimization_results =
        run_firefly_swarm_optimization(problem, optimized_hyperparameters)?;

    let problem_delta_time = problem_start_time.elapsed().as_secs_f64();


    let formatted_parameters = optimization_results
        .minimum
        .vector
        .iter()
        .map(|parameter| parameter.to_string())
        .join(",");

    println!();
    println!();
    println!(
        "Problem {:02}/{:02} ({}) optimized in {:.4} seconds.",
        bbob_function.index(),
        ALL_BBOB_FUNCTIONS.len(),
        bbob_function.name(),
        problem_delta_time,
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
