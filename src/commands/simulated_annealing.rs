use std::time::Instant;
use itertools::Itertools;
use miette::Result;
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::algorithms::simulated_annealing::simulated_annealing::run_sa;
use crate::core::functions::ALL_BBOB_FUNCTION_NAMES;
use crate::core::suite::BBOBSuite;

pub fn run_cmd_simulated_annealing() -> Result<()> {
    let mut suite = BBOBSuite::new()?;

    let total_start_time = Instant::now();

    let run_options = SAOptions::default();

    // Run all 24 BBOB problems.
    for bbob_function in ALL_BBOB_FUNCTION_NAMES {
        let mut problem = suite.problem(bbob_function, None)?;
        let problem_start_time = Instant::now();

        let results = run_sa(
            problem,
            run_options.clone(),
        )?;

        let problem_delta_time = problem_start_time.elapsed().as_secs_f64();

        let formatted_parameters = results
            .vector
            .iter()
            .map(|parameter| parameter.to_string())
            .join(",");

        println!(
            "[Problem {:02}/{:02}: {}] - {:.4} seconds",
            bbob_function.index(),
            ALL_BBOB_FUNCTION_NAMES.len(),
            bbob_function.name(),
            //results.iterations_performed,
            //run_options.maximum_iterations,
            problem_delta_time
        );

        println!("  Minimum: {}", results.value,);
        println!("  At: [{}]", formatted_parameters);

        println!(
            "  Distance from global minimum: {:.5}",
            results.value - bbob_function.global_minimum()
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