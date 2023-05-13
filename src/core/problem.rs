use coco_rs::Problem;
use miette::{miette, Result};

pub struct BBOBProblem<'suite> {
    problem: Problem<'suite>,
}

impl<'suite> BBOBProblem<'suite> {
    pub fn from_coco_problem(problem: Problem<'suite>) -> Result<Self> {
        if problem.dimension() != 40 {
            return Err(miette!("Problem doesn't have 40 dimensions!"));
        }

        if problem.number_of_objectives() > 1 {
            return Err(miette!("Problem has more than one objective!"));
        }

        Ok(Self { problem })
    }

    pub fn evaluate(&mut self, input: &[f64]) -> f64 {
        // Safety: problem.number_of_objectives() is guaranteed to be 1 on initialization.
        let mut values = vec![0f64; 1];

        self.problem.evaluate_function(input, &mut values);

        values[0]
    }

    #[allow(dead_code)]
    pub fn inner_problem(&self) -> &Problem<'suite> {
        &self.problem
    }
}
