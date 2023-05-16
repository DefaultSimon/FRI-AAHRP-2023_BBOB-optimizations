use std::time::Instant;

use itertools::Itertools;
use miette::Result;

use crate::algorithms::firefly::{
    perform_firefly_swarm_optimization,
    FireflyOptions,
};
use crate::core::functions::ALL_BBOB_FUNCTION_NAMES;
use crate::core::suite::BBOBSuite;

pub fn cmd_run_firefly_optimization() -> Result<()> {
    // Initialize coco / BBOB suite.
    let mut suite = BBOBSuite::new()?;

    let total_start_time = Instant::now();

    let run_options = FireflyOptions::default();

    // Run all 24 BBOB problems.
    // TODO We can actually parallelize this by running multiple individual problems at the same time.
    for bbob_function in ALL_BBOB_FUNCTION_NAMES {
        let problem = suite.problem(bbob_function, None)?;
        let problem_start_time = Instant::now();

        let results = perform_firefly_swarm_optimization(
            problem,
            Some(run_options.clone()),
        )?;

        let problem_delta_time = problem_start_time.elapsed().as_secs_f64();

        let formatted_parameters = results
            .minimum
            .vector
            .iter()
            .map(|parameter| parameter.to_string())
            .join(",");

        println!(
            "[Problem {:02}/{:02}: {}] - {}/{} iterations, {:.4} seconds",
            bbob_function.index(),
            ALL_BBOB_FUNCTION_NAMES.len(),
            bbob_function.name(),
            results.iterations_performed,
            run_options.maximum_iterations,
            problem_delta_time
        );

        println!("  Minimum: {}", results.minimum.value,);
        println!("  At: [{}]", formatted_parameters);

        println!(
            "  Distance from global minimum: {:.5}",
            results.minimum.value - bbob_function.global_minimum()
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
