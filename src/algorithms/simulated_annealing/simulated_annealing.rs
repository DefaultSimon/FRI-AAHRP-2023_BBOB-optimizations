use std::borrow::BorrowMut;
use std::f64::consts::E;

use miette::{miette, Result};

use crate::algorithms::common::rng::{choose_random, SimpleUniformRng, UniformF64BoundedRandomGenerator};
use crate::algorithms::common::structs::State;
use crate::algorithms::simulated_annealing::neighborhood_generation::{LocalSearchNeighborhood, SANeighborhood};
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::core::problem::BBOBProblem;

use super::super::common::structs::Minimum;

static NEIGHBORHOOD_SIZE: u64 = 100;

pub fn run_sa(problem: &mut BBOBProblem, options: SAOptions) -> Result<Minimum> {
    let mut rng = UniformF64BoundedRandomGenerator::new(problem.bounds, options.seed);
    let mut current_state = State {
        vector: rng.sample_multiple(40),
        ..Default::default()
    };
    let mut minimal_state = current_state.clone();
    current_state.set_objective_value(problem.evaluate(&current_state.vector));
    let mut neighborhood = &mut SANeighborhood::new(problem.bounds, options.seed);
    let mut temperature = options.initial_temperature as f64;
    let mut iters = 0;

    while temperature > options.min_temp ||  iters > options.max_iterations_sa {
        neighborhood.generate_neighborhood(current_state.clone(), problem, options);
        let next_state = &mut choose_random(neighborhood.states.clone());
        next_state.set_objective_value(problem.evaluate(&next_state.vector));

        if next_state.objective_value < minimal_state.objective_value {
            minimal_state = next_state.clone();
        } else {
            let p_move = (-(next_state.objective_value - current_state.objective_value) / temperature).exp();
            if rng.sample_0_to_1() <= p_move {
                current_state = next_state.clone();
            }
            temperature = options.annealing_schedule * temperature;
        }
        iters += 1;
    }
    local_search(problem, &minimal_state, options)
}

fn local_search(
    mut problem: &mut BBOBProblem,
    start_state: &State,
    options: SAOptions
) -> Result<Minimum> {
    let mut current_state = start_state.clone();
    let mut minimal_state = current_state.clone();
    let mut neighborhood = &mut LocalSearchNeighborhood::new(problem.bounds, options.seed);
    let mut iters = 0;

    while iters <  options.max_iterations_ls {
        neighborhood.generate_neighborhood(minimal_state.clone(), problem, options);
        for el in neighborhood.states.iter() {
            let mut moved = el.clone();
            moved.set_objective_value(problem.evaluate(&el.vector));
            if moved.objective_value < minimal_state.objective_value {
                minimal_state = moved.clone();
            }
        }
        iters += 1;
    }

    Ok(Minimum { vector: minimal_state.vector, value: minimal_state.objective_value })
}