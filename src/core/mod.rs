pub mod names;

use coco_rs::{Problem, Suite, SuiteName};
use miette::{miette, Result};

use crate::core::names::BBOBFunctionName;

pub struct BBOBSuite {
    suite: Suite,
}

impl BBOBSuite {
    pub fn new() -> Result<Self> {
        let suite = Suite::new(
            SuiteName::Bbob,
            "year: 2009, instances: 2023",
            "dimensions: 40, function_indices: 1-24",
        )
        .ok_or_else(|| miette!("Could not initialize BBOX suite!"))?;

        Ok(Self { suite })
    }

    pub fn problem(
        &mut self,
        bbob_function: BBOBFunctionName,
    ) -> Result<BBOBProblem> {
        let raw_problem = self
            .suite
            .problem_by_function_dimension_instance(
                bbob_function.to_function_index(),
                40,
                2023,
            )
            .ok_or_else(|| miette!("Could not get BBOX problem!"))?;

        BBOBProblem::from_coco_problem(raw_problem)
    }
}

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
