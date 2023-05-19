use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

use itertools::Itertools;
use miette::Result;

use crate::algorithms::simulated_annealing::hyperparam_optimization::get_optimal_params;
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::algorithms::simulated_annealing::simulated_annealing::run_sa;
use crate::core::functions::ALL_BBOB_FUNCTIONS;
use crate::core::suite::BBOBSuite;

pub fn run_cmd_simulated_annealing() -> Result<()> {
    let mut suite = BBOBSuite::new()?;

    let total_start_time = Instant::now();

    let run_options = SAOptions::default();

    // Run all 24 BBOB problems.
    // for bbob_function in ALL_BBOB_FUNCTIONS {
    //     let mut problem = suite.problem(bbob_function, None)?;
    //     let problem_start_time = Instant::now();
    //
    //     let results = run_sa(&mut problem, run_options)?;
    //
    //     let problem_delta_time = problem_start_time.elapsed().as_secs_f64();
    //
    //     let formatted_parameters = results
    //         .vector
    //         .iter()
    //         .map(|parameter| parameter.to_string())
    //         .join(",");
    //
    //     println!(
    //         "[Problem {:02}/{:02}: {}] - {:.4} seconds",
    //         bbob_function.index(),
    //         ALL_BBOB_FUNCTIONS.len(),
    //         bbob_function.name(),
    //         //results.iterations_performed,
    //         //run_options.maximum_iterations,
    //         problem_delta_time
    //     );
    //
    //     println!("  Minimum: {}", results.value, );
    //     println!("  At: [{}]", formatted_parameters);
    //
    //     println!(
    //         "  Distance from global minimum: {:.5}",
    //         results.value - bbob_function.global_minimum()
    //     );
    //     println!();
    // }
    //
    // let total_delta_time = total_start_time.elapsed().as_secs_f64();
    //
    // println!(
    //     "\n-- Finished all 24 problems in {:.4} seconds --",
    //     total_delta_time
    // );
    let mut handles = Vec::new();
    for bbob_function in ALL_BBOB_FUNCTIONS.as_ref() {
        let mut suite_inner = BBOBSuite::new().unwrap();
        handles.push(thread::spawn(move || {
            let mut problem = suite_inner.problem(*bbob_function, None).unwrap();
            println!("Finding optimal parameters:");
            get_optimal_params(&mut problem)
        }));
    }

    for jh in handles.into_iter() {
        let options = jh.join().unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("opitons.txt")
            .unwrap();
        file.write(format!("{}Options {}", options.function.name(), options.to_str()).as_bytes()).expect("Unable to write to file");
    }
    Ok(())
}
