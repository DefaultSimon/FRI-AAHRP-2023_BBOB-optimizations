use coco_rs::Problem;
use miette::{miette, Result};
use rand::distributions::Uniform;

use crate::core::names::BBOBFunctionName;

#[derive(Copy, Clone, PartialEq)]
pub struct Bounds {
    lower_bound: f64,
    upper_bound: f64,
}

impl Bounds {
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            lower_bound,
            upper_bound,
        }
    }

    pub fn uniform_random_generator(&self) -> Uniform<f64> {
        Uniform::new_inclusive(self.lower_bound, self.upper_bound)
    }
}

pub struct BBOBProblem<'suite> {
    problem: Problem<'suite>,
    pub name: BBOBFunctionName,
    bounds: Bounds,
}

impl<'suite> BBOBProblem<'suite> {
    pub fn from_problem_and_name(
        problem: Problem<'suite>,
        function_name: BBOBFunctionName,
        bounds: Bounds,
    ) -> Result<Self> {
        if problem.dimension() != 40 {
            return Err(miette!("Problem doesn't have 40 dimensions!"));
        }

        if problem.number_of_objectives() > 1 {
            return Err(miette!("Problem has more than one objective!"));
        }

        Ok(Self {
            problem,
            name: function_name,
            bounds,
        })
    }

    pub fn evaluate(&mut self, input: &[f64]) -> f64 {
        // Safety: problem.number_of_objectives() is guaranteed to be 1 on initialization.
        let mut values = vec![0f64; 1];

        self.problem.evaluate_function(input, &mut values);

        values[0]
    }

    pub fn input_dimensions(&self) -> usize {
        self.problem.dimension()
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    #[allow(dead_code)]
    pub fn inner_problem(&self) -> &Problem<'suite> {
        &self.problem
    }
}
