use std::time::Instant;

use itertools::Itertools;
use miette::Result;

use crate::algorithms::firefly::{
    perform_firefly_swarm_optimization,
    FullFireflyOptions,
    RunFireflyOptions,
};
use crate::core::functions::{BBOBFunction, ALL_BBOB_FUNCTION_NAMES};
use crate::core::suite::BBOBSuite;

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

pub fn cmd_run_firefly_optimization() -> Result<()> {
    // Initialize coco / BBOB suite.
    let mut suite = BBOBSuite::new()?;

    let total_start_time = Instant::now();

    // Run all 24 BBOB problems.
    // TODO We can actually parallelize this by running multiple individual problems at the same time.
    for bbob_function in ALL_BBOB_FUNCTION_NAMES {
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
            ALL_BBOB_FUNCTION_NAMES.len(),
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
        "\n-- Finished all 24 problems in {:.4} seconds --",
        total_delta_time
    );

    Ok(())
}
