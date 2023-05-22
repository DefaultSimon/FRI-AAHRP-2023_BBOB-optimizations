use miette::Result;
use num::abs;

use super::super::common::structs::Minimum;
use crate::algorithms::common::rng::{
    choose_random,
    UniformF64BoundedRandomGenerator,
};
use crate::algorithms::common::structs::State;
use crate::algorithms::simulated_annealing::neighborhood_generation::{
    LocalSearchNeighborhood,
    SANeighborhood,
};
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::core::problem::BBOBProblem;

pub fn run_sa(problem: &mut BBOBProblem, options: SAOptions) -> Result<Minimum> {
    let mut rng =
        UniformF64BoundedRandomGenerator::new(problem.bounds, options.seed);

    let mut current_state = State::new_without_value(rng.sample_multiple(40));
    let mut minimal_state = current_state.clone();
    current_state.set_objective_value(problem.evaluate(&current_state.vector));

    let neighborhood = &mut SANeighborhood::new();
    let mut temperature = options.initial_temperature as f64;
    let mut iters = 0;

    while temperature > options.min_temp || iters < options.max_iterations_sa {
        neighborhood.generate_neighborhood(
            current_state.clone(),
            problem,
            options,
        );
        let next_state = &mut choose_random(neighborhood.states.clone());
        next_state.set_objective_value(problem.evaluate(&next_state.vector));

        if next_state.objective_value < minimal_state.objective_value {
            minimal_state = next_state.clone();
        } else {
            let p_move = (-(next_state.objective_value
                - current_state.objective_value)
                / temperature)
                .exp();
            if rng.sample_0_to_1() <= p_move {
                current_state = next_state.clone();
            }
            temperature *= options.annealing_schedule;
        }

        iters += 1;
    }
    local_search(problem, &minimal_state, options)
}

fn local_search(
    problem: &mut BBOBProblem,
    start_state: &State,
    options: SAOptions,
) -> Result<Minimum> {
    let current_state = start_state.clone();
    let mut minimal_state = current_state;

    let neighborhood = &mut LocalSearchNeighborhood::new();

    let mut iters = 0;
    let mut step = options.initial_step_size_ls;

    let mut last_10_values = Vec::new();
    last_10_values.resize(10, 0f64);

    let mut current_options = options;

    while iters < options.max_iterations_ls && step >= 10e-16 {
        neighborhood.generate_neighborhood(
            minimal_state.clone(),
            problem,
            current_options,
        );

        for el in neighborhood.states.iter() {
            let mut moved = el.clone();
            let objective_value = problem.evaluate(&el.vector);

            last_10_values[(iters % 10) as usize] = objective_value;
            moved.set_objective_value(objective_value);
            if moved.objective_value < minimal_state.objective_value {
                minimal_state = moved.clone();
            }
        }

        last_10_values[(iters % 10) as usize] = minimal_state.objective_value;

        if check_last_10_similar(&last_10_values) {
            if step <= 1f64 {
                step *= options.ls_step_decrease;
            } else {
                step -= options.ls_step_decrease;
            }
            current_options = SAOptions {
                initial_step_size_ls: step,
                ..current_options
            };
        }

        iters += 1;
    }

    Ok(Minimum {
        vector: minimal_state.vector,
        value: minimal_state.objective_value,
    })
}

fn check_last_10_similar(last_10: &Vec<f64>) -> bool {
    if last_10.len() < 10 {
        return false;
    }
    let fst = last_10[0];

    for el in last_10.iter() {
        if abs(*el - fst) > 10e-3 {
            return false;
        }
    }

    true
}
