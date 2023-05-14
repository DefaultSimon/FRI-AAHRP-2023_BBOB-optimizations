use crate::core::problem::BBOBProblem;

use super::common::Minimum;

use miette::{Result, miette};

pub fn run_sa<'a>(problem: &'a BBOBProblem, temperature: f64, annealing_schedule: f64) -> Result<Minimum<'a>> {
    Err(miette!("Not implemented!"))
}
