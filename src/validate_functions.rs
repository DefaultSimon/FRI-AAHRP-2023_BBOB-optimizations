use std::collections::HashMap;
use std::env;
use std::env::current_dir;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;

use aahrp_2023_bbob_optimizations::core::functions::BBOBFunction;
use aahrp_2023_bbob_optimizations::core::suite::BBOBSuite;
use miette::{miette, Context, IntoDiagnostic, Result};
use regex::Regex;


#[inline]
fn f64_approximate_eq(first: f64, second: f64, max_distance: f64) -> bool {
    (first - second).abs() < max_distance
}

static DEFAULT_F64_EQ_DISTANCE: f64 = 0.0001;

pub struct Sample {
    function_index: usize,
    function_parameters: Vec<f64>,
    function_value: f64,
}

impl Sample {
    pub fn new(index: usize, parameters: Vec<f64>, value: f64) -> Self {
        Self {
            function_index: index,
            function_parameters: parameters,
            function_value: value,
        }
    }
}

impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        if self.function_index != other.function_index {
            return false;
        }

        for (self_parameter, other_parameter) in self
            .function_parameters
            .iter()
            .zip(other.function_parameters.iter())
        {
            if !f64_approximate_eq(
                *self_parameter,
                *other_parameter,
                DEFAULT_F64_EQ_DISTANCE,
            ) {
                return false;
            }
        }

        f64_approximate_eq(
            self.function_value,
            other.function_value,
            DEFAULT_F64_EQ_DISTANCE,
        )
    }
}

impl Eq for Sample {}


pub struct FunctionSamples {
    samples_by_function_index: HashMap<usize, Vec<Sample>>,
}

impl FunctionSamples {
    pub fn from_index_parameter_value_triplets(
        triplets: Vec<(usize, Vec<f64>, f64)>,
    ) -> Self {
        let mut samples_by_function_index: HashMap<usize, Vec<Sample>> =
            HashMap::new();

        for (index, parameter, value) in triplets {
            let sample = Sample::new(index, parameter, value);

            let samples_per_index =
                samples_by_function_index.entry(index).or_default();
            samples_per_index.push(sample);
        }

        Self {
            samples_by_function_index,
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

fn run_r_script() -> Result<FunctionSamples> {
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
        .output()
        .into_diagnostic()?;

    if !r_output.status.success() {
        return Err(miette!(
            "Rscript call failed with status code {}. Stderr:\n{}",
            r_output.status.code().unwrap_or(-1),
            from_utf8(&r_output.stderr).into_diagnostic()?
        ));
    }

    let rscript_stdout_output = from_utf8(&r_output.stdout).into_diagnostic()?;

    // Parse stdout output of the R script.
    let extraction_regex = Regex::new(
        r#"bbob_function_index=(?P<index>\d+);parameters=\[(?P<parameters>(-?\d+(?:\.?\d+)?)(,-?\d+(?:\.?\d+)?)*)\];value=(?P<value>\d+\.?\d*)"#,
    ).expect("Invalid static regex?!");

    let mut index_parameter_value_triplets: Vec<(usize, Vec<f64>, f64)> =
        Vec::new();

    for line in rscript_stdout_output.lines() {
        let capture_groups = match extraction_regex.captures(line) {
            Some(captures) => captures,
            // Skips non-matching lines
            None => continue,
        };

        let bbob_function_index: usize = {
            let index_group = capture_groups
                .name("index")
                .ok_or_else(|| miette!("Missing capture group: index!"))?;

            index_group
                .as_str()
                .parse::<usize>()
                .into_diagnostic()
                .wrap_err_with(|| {
                    miette!("Could not convert index str to usize!")
                })?
        };

        let parameters: Vec<f64> = {
            let parameters_group = capture_groups
                .name("parameters")
                .ok_or_else(|| miette!("Missing capture group: parameters!"))?;

            parameters_group
                .as_str()
                .split(',')
                .map(|parameter_str| {
                    parameter_str
                        .parse::<f64>()
                        .into_diagnostic()
                        .wrap_err_with(|| {
                            miette!("Could not parse f64 in parameter list!")
                        })
                })
                .collect::<Result<Vec<f64>>>()?
        };

        let value: f64 = {
            let value_group = capture_groups
                .name("value")
                .ok_or_else(|| miette!("Missing capture group: value!"))?;

            value_group
                .as_str()
                .parse::<f64>()
                .into_diagnostic()
                .wrap_err_with(|| {
                    miette!("Could not convert value str to f64!")
                })?
        };

        index_parameter_value_triplets.push((
            bbob_function_index,
            parameters,
            value,
        ));
    }

    Ok(
        FunctionSamples::from_index_parameter_value_triplets(
            index_parameter_value_triplets,
        ),
    )
}

fn compare_r_with_rust(r_samples: FunctionSamples) -> Result<()> {
    let mut suite = BBOBSuite::new()?;

    let mut sorted_samples: Vec<(usize, Vec<Sample>)> =
        r_samples.samples_by_function_index.into_iter().collect();
    sorted_samples.sort_unstable_by(
        |(function_index, _), (other_function_index, _)| {
            function_index.cmp(other_function_index)
        },
    );

    for (function_index, samples) in sorted_samples {
        let mut problem = suite.problem(
            BBOBFunction::from_function_index(function_index).ok_or_else(
                || miette!("Invalid function index! Not in 1-24 range."),
            )?,
            None,
        )?;

        for (sample_index, sample) in samples.iter().enumerate() {
            let value = problem.evaluate(&sample.function_parameters);

            if f64_approximate_eq(
                value,
                sample.function_value,
                DEFAULT_F64_EQ_DISTANCE,
            ) {
                println!(
                    "Function {}, sample {}: OK ({:.4} (R) == {:.4} (Rust))",
                    function_index,
                    sample_index + 1,
                    sample.function_value,
                    value,
                );
            } else {
                println!(
                    "Function {}, sample {}: FAILED ({:.4} (R) != {:.4} (Rust))",
                    function_index,
                    sample_index + 1,
                    sample.function_value,
                    value,
                );

                return Err(miette!("One of the comparisons failed!"));
            }
        }
    }

    println!("\nDONE! All comparisons OK.");

    Ok(())
}

fn main() -> Result<()> {
    let r_function_results = run_r_script()?;
    compare_r_with_rust(r_function_results)?;

    Ok(())
}
