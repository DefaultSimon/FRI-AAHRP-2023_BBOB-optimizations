use miette::Result;

use crate::algorithms::firefly::perform_firefly_swarm_optimization;
use crate::core::names::ALL_BBOB_FUNCTION_NAMES;
use crate::core::suite::BBOBSuite;

pub fn cmd_run_firefly_optimization() -> Result<()> {
    let mut suite = BBOBSuite::new()?;

    for problem_name in ALL_BBOB_FUNCTION_NAMES {
        let problem = suite.problem(problem_name, None)?;

        let minimum = perform_firefly_swarm_optimization(problem, None)?;
        println!(
            "[{:02}/{:02}|{}] Minimum: {:.6}",
            problem_name.to_function_index(),
            ALL_BBOB_FUNCTION_NAMES.len(),
            problem_name.to_function_name(),
            minimum.value,
        );
    }

    Ok(())
}
