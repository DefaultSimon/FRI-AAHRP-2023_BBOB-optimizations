use coco_rs::{LogLevel, Suite, SuiteName};

/// This command is named `run-R-smoof-comparison` and is intended to be paired with
/// the `smoof_comparison.R` R script - if they match, the 1-24 functions are the same
/// as in the `smoof` R package (in `makeBBOBFunction`).
pub fn cmd_smoof_comparison() {
    coco_rs::set_log_level(LogLevel::Error);

    let mut bbob_suite = Suite::new(
        SuiteName::Bbob,
        "year: 2009, instances: 2023",
        "dimensions: 40, function_indices: 1-24",
    )
    .expect("Could not initialize BBOB suite.");

    println!("The output of this script should be compared with the output of the `smoof_comparison.R` script.");
    println!("Calculating all 24 BBOB functions at point [4, 4, 4, ...] (4 in all 40 dimensions).");

    let mut function_index = 1;
    loop {
        let mut next_problem = match bbob_suite.next_problem(None) {
            None => break,
            Some(problem) => problem,
        };

        let input_values = vec![4f64; 40];
        let mut output_values = vec![0f64; next_problem.number_of_objectives()];
        next_problem.evaluate_function(&input_values, &mut output_values);

        println!(
            "Function {}: {:.4}",
            function_index, output_values[0]
        );

        function_index += 1;
    }

    println!("Finished.");
}
