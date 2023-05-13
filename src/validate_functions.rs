use std::env;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;

use aahrp_2023_bbob_optimizations::core::names::ALL_BBOB_FUNCTION_NAMES;
use aahrp_2023_bbob_optimizations::core::suite::BBOBSuite;
use miette::{miette, Context, IntoDiagnostic, Result};
use regex::Regex;


#[inline]
fn f64_approximate_eq(first: f64, second: f64, max_distance: f64) -> bool {
    (first - second).abs() < max_distance
}

static DEFAULT_F64_EQ_DISTANCE: f64 = 0.00001;


pub struct FunctionResults {
    values_per_function: Vec<(usize, f64)>,
}

impl FunctionResults {
    pub fn from_index_value_pairs(pairs: Vec<(usize, f64)>) -> Self {
        Self {
            values_per_function: pairs,
        }
    }
}


fn find_rscript_binary() -> Result<PathBuf> {
    let rscript_in_path = which::which("Rscript");
    if let Ok(rscript) = rscript_in_path {
        Ok(dunce::canonicalize(rscript).into_diagnostic()?)
    } else {
        let rscript_in_standard_location = which::which_in(
            "RScript",
            match env::consts::OS {
                "windows" => Some("C:\\Program Files\\R\\R-4.3.0\\bin\\x64"),
                "macos" => Some("/Applications/R.app/Contents/MacOS/R"),
                _ => None,
            },
            current_dir().into_diagnostic()?,
        );

        if let Ok(rscript) = rscript_in_standard_location {
            Ok(dunce::canonicalize(rscript).into_diagnostic()?)
        } else {
            Err(miette!(
                "Could not find Rscript in PATH or standard R locations."
            ))
        }
    }
}

fn run_r_script() -> Result<FunctionResults> {
    // Find R installation.
    let rscript_binary = find_rscript_binary()?;
    println!("Using Rscript binary at {:?}", rscript_binary);

    // Prepare paths.
    let smoof_comparison_script = Path::new("./smoof_comparison.R")
        .canonicalize()
        .into_diagnostic()?;

    if !(smoof_comparison_script.exists() && smoof_comparison_script.is_file()) {
        return Err(miette!(
            "Missing smoof_comparison.R file in current directory!"
        ));
    }

    // Run `Rscript smoof_comparison.R` to get R's `smoof` library output.
    let r_output = Command::new(rscript_binary)
        .arg(smoof_comparison_script.to_string_lossy().to_string())
        .current_dir(current_dir().into_diagnostic()?)
        .output()
        .into_diagnostic()?;

    if !r_output.status.success() {
        return Err(miette!(
            "Rscript call failed with status code {}. Stderr: {}",
            r_output.status.code().unwrap_or(-1),
            from_utf8(&r_output.stderr).into_diagnostic()?
        ));
    }

    let rscript_stdout_output = from_utf8(&r_output.stdout).into_diagnostic()?;

    // Parse stdout output of the R script.
    let extraction_regex = Regex::new(
        r#"\[1\] "Function (?P<index>\d+): (?P<value>-?\d+(\.\d+)?)""#,
    )
    .expect("Invalid regex!?");

    let function_index_and_value_pairs = rscript_stdout_output
        .lines()
        .filter_map(|line| {
            let capture_groups = match extraction_regex.captures(line) {
                Some(groups) => groups,
                None => return None,
            };

            let function_index = match capture_groups.name("index") {
                Some(index) => match index.as_str().parse::<usize>() {
                    Ok(index) => index,
                    Err(error) => {
                        return Some(Err(error).into_diagnostic().wrap_err_with(
                            || {
                                miette!(
                                    "Could not convert string to usize: {}",
                                    index.as_str()
                                )
                            },
                        ))
                    }
                },
                None => {
                    return Some(Err(miette!("No function index in output!")));
                }
            };
            let function_value = match capture_groups.name("value") {
                Some(value) => match value.as_str().parse::<f64>() {
                    Ok(value) => value,
                    Err(error) => {
                        return Some(Err(error).into_diagnostic().wrap_err_with(
                            || {
                                miette!(
                                    "Could not convert string to f64: {}",
                                    value.as_str()
                                )
                            },
                        ))
                    }
                },
                None => {
                    return Some(Err(miette!("No function value in output!")));
                }
            };

            Some(Ok((function_index, function_value)))
        })
        .collect::<Result<Vec<(usize, f64)>>>()?;

    Ok(FunctionResults::from_index_value_pairs(
        function_index_and_value_pairs,
    ))
}

fn run_rust_script() -> Result<FunctionResults> {
    let mut suite = BBOBSuite::new()?;

    let input_values = vec![4f64; 40];

    let function_index_and_value_pairs = ALL_BBOB_FUNCTION_NAMES
        .into_iter()
        .enumerate()
        .map(|(function_index, function_name)| {
            let mut problem = suite.problem(function_name)?;
            let value = problem.evaluate(&input_values);

            Ok((function_index + 1, value))
        })
        .collect::<Result<Vec<(usize, f64)>>>()?;

    Ok(FunctionResults::from_index_value_pairs(
        function_index_and_value_pairs,
    ))
}

fn main() -> Result<()> {
    let r_function_results = run_r_script()?;
    let rust_function_results = run_rust_script()?;

    // Compare results
    for (function_index, r_function_value) in
        r_function_results.values_per_function
    {
        let rust_function_value = rust_function_results
            .values_per_function
            .iter()
            .find_map(|(index, value)| {
                if function_index.eq(index) {
                    Some(value)
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                miette!(
                    "No such function index in Rust test: {}",
                    function_index
                )
            })?;

        if f64_approximate_eq(
            r_function_value,
            *rust_function_value,
            DEFAULT_F64_EQ_DISTANCE,
        ) {
            println!(
                "Function {}: OK ({:.5} (R) == {:.5} (Rust))",
                function_index, r_function_value, rust_function_value
            );
        } else {
            println!(
                "Function {}: MISMATCH ({:.5} (R) != {:.5} (Rust))",
                function_index, r_function_value, rust_function_value
            )
        }
    }

    Ok(())
}
