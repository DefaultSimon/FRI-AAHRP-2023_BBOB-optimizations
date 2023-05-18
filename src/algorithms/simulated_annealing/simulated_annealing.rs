use std::f64::consts::E;

use miette::{miette, Result};

use super::super::common::structs::Minimum;
use crate::algorithms::common::rng::{choose_random, SimpleUniformRng, UniformF64BoundedRandomGenerator};
use crate::algorithms::common::structs::State;
use crate::algorithms::simulated_annealing::neighborhood_generation::{SANeighborhood};
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::core::problem::BBOBProblem;

static NEIGHBORHOOD_SIZE: u64 = 100;

pub fn run_sa(mut problem: BBOBProblem, options: SAOptions) -> Result<Minimum> {
    let mut rng = SimpleUniformRng::new(0f64, 1f64);
    let mut current_state = State {
        vector: rng.sample_multiple(40), ..Default::default()
    };
    let mut minimal_state = current_state.clone();
    current_state.set_objective_value(problem.evaluate(&current_state.vector));
    let mut neighborhood = &mut SANeighborhood::new();
    neighborhood.generate_neighborhood(&current_state);
    let mut temperature = options.temperature as f64;

    while temperature > 0.1 {
        let next_state = &mut choose_random(neighborhood.states.clone());
        next_state.set_objective_value(problem.evaluate(&next_state.vector));

        if next_state.objective_value < minimal_state.objective_value {
            minimal_state = next_state.clone();
        } else {
            let p_move = E.powf((next_state.objective_value - minimal_state.objective_value) / temperature);
            if rng.sample() <= p_move {
                current_state = next_state.clone();
            }
            temperature = options.annealing_schedule * temperature;
        }
    }
    local_search(problem, &minimal_state)
}

fn local_search(
    mut problem: BBOBProblem,
    start_state: &State,
) -> Result<Minimum> {
    let min = Minimum {vector: start_state.clone().vector, value: problem.evaluate(&start_state.clone().vector)};
    Ok(min)
}