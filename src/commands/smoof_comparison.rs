use miette::Result;

use crate::core::names::ALL_BBOB_FUNCTION_NAMES;
use crate::core::BBOBSuite;

/// This command is named `run-R-smoof-comparison` and is intended to be paired with
/// the `smoof_comparison.R` R script - if they match, the 1-24 functions are the same
/// as in the `smoof` R package (in `makeBBOBFunction`).
pub fn cmd_smoof_comparison() -> Result<()> {
    let mut suite = BBOBSuite::new()?;

    println!(
        "The output of this script should be compared with the output of \
        the `smoof_comparison.R` script."
    );
    println!(
        "Calculating all 24 BBOB functions at point [4, 4, 4, ...] \
        (4 in all 40 dimensions)."
    );

    for (index, bbob_function) in ALL_BBOB_FUNCTION_NAMES.into_iter().enumerate()
    {
        let mut problem = suite.problem(bbob_function)?;

        let input_values = vec![4f64; 40];
        let value = problem.evaluate(&input_values);

        println!("Function {}: {:.4}", index + 1, value);
    }

    println!("Finished.");
    Ok(())
}
