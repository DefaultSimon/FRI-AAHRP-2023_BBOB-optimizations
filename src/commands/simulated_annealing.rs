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
    let total_start_time = Instant::now();

    let mut minimums = Vec::new();
    let mut vectors = Vec::new();

    // Run all 24 BBOB problems.
    for bbob_function in ALL_BBOB_FUNCTIONS {
        let mut options = SAOptions::default();

        let mut fun_values = Vec::new();
        let mut min_vectors = Vec::new();

        for _ in 0..20 {
            let mut suite = BBOBSuite::new()?;
            let mut problem = suite.problem(bbob_function)?;

            let problem_start_time = Instant::now();

            let results = run_sa(&mut problem, options)?;
            fun_values.push(results.value);
            min_vectors.push(results.vector.clone());

            let problem_delta_time = problem_start_time.elapsed().as_secs_f64();

            let formatted_parameters = results
                .vector
                .iter()
                .map(|parameter| parameter.to_string())
                .join(",");

            println!(
                "[Problem {:02}/{:02}: {}] - {:.4} seconds",
                bbob_function.index(),
                ALL_BBOB_FUNCTIONS.len(),
                bbob_function.name(),
                problem_delta_time
            );

            println!("  Minimum: {}", results.value,);
            println!("  At: [{}]", formatted_parameters);

            println!(
                "  Distance from global minimum: {:.5}",
                results.value - bbob_function.global_minimum()
            );
            println!();
            println!("Finding optimal params:");
            options = get_optimal_params(&mut problem, options);
        }

        let mut min = f64::MAX;
        let mut min_index = 0;
        for (i, el) in fun_values.iter().enumerate() {
            if *el < min {
                min = *el;
                min_index = i;
            }
        }

        minimums.push(min);
        vectors.push(min_vectors[min_index].clone());
    }

    let total_delta_time = total_start_time.elapsed().as_secs_f64();

    println!(
        "\n-- Finished all 24 problems in {:.4} seconds --",
        total_delta_time
    );

    println!("Minimum values:");

    for (i, el) in minimums.iter().enumerate() {
        println!("{}: {}", ALL_BBOB_FUNCTIONS[i].name(), el);
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("results/simulated-annealing-results.txt")
        .unwrap();

    for vec in vectors.iter() {
        file.write_all(
            vec.iter().map(|el| el.to_string()).join("\t").as_bytes(),
        )
        .unwrap();

        file.write_all("\n".as_bytes()).unwrap();
    }

    // let mut handles = Vec::new();
    // for bbob_function in ALL_BBOB_FUNCTIONS.as_ref() {
    //     let mut suite_inner = BBOBSuite::new().unwrap();
    //     handles.push(thread::spawn(move || {
    //         let mut problem = suite_inner.problem(*bbob_function, None).unwrap();
    //         println!("Finding optimal parameters:");
    //         get_optimal_params(&mut problem)
    //     }));
    // }
    //
    // for jh in handles.into_iter() {
    //     let options = jh.join().unwrap();
    //     let mut file = OpenOptions::new()
    //         .create(true)
    //         .append(true)
    //         .open("opitons.txt")
    //         .unwrap();
    //     file.write(format!("{}Options {}", options.function.name(), options.to_str()).as_bytes()).expect("Unable to write to file");
    // }
    Ok(())
}
