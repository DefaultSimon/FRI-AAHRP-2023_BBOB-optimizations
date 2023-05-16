use coco_rs::{Suite, SuiteName};
use miette::{miette, Result};

use crate::core::functions::BBOBFunction;
use crate::core::problem::{BBOBProblem, Bounds};

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
        bbob_function: BBOBFunction,
        bounds: Option<Bounds>,
    ) -> Result<BBOBProblem> {
        let raw_problem = self
            .suite
            .problem_by_function_dimension_instance(
                bbob_function.index(),
                40,
                2023,
            )
            .ok_or_else(|| miette!("Could not get BBOX problem!"))?;

        let bounds = bounds.unwrap_or_else(|| Bounds::new(-5f64, 5f64));

        BBOBProblem::from_problem_and_name(raw_problem, bbob_function, bounds)
    }
}
