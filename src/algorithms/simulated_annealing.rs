use std::{ops::Add, ptr::null, f64::consts::E};

use crate::core::problem::BBOBProblem;
use super::common::{Minimum};
use miette::{Result, miette};
use rand::{rngs::*, thread_rng, Rng};

static NEIGHBORHOOD_SIZE: u64 = 100;

pub fn run_sa<'a>(problem: &mut BBOBProblem, mut temperature: f64, annealing_schedule: f64) -> Result<Minimum> {
    let mut current_state = State { vector: generate_random_vec() };
    let mut minimal_state = current_state.clone();
    let mut minimal_value = problem.evaluate(&current_state.vector);
    let neighborhood = generate_neighborhood(current_state);

    while temperature > 0.1 {
        let next_state = random_choice(&neighborhood);
        let result = problem.evaluate(&next_state.vector);

        if (result < minimal_value) {
            minimal_state = next_state;
            minimal_value = result;
        } else {
            let p_move = E.powf((result - minimal_value) / temperature);
            let mut rng = thread_rng();
            if rng.gen::<f64>() <= p_move {
                current_state = next_state;
            }
            temperature = annealing_schedule * temperature;
        }
    }
    local_search(problem, minimal_state)
}

fn local_search(problem: &mut BBOBProblem, start_state: State) -> Result<Minimum> {
    Err(miette!("Not today"))
}

#[derive(Debug, Clone)]
struct State {
    pub vector: Vec<f64>
}

#[derive(Debug)]
struct Neighborhood {
    pub states: Vec<State>
}

fn random_choice<'a>(neighborhood: &'a Neighborhood) -> State {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range::<usize, _>(0..neighborhood.states.len());
    neighborhood.states[rand_num].clone()
}

fn generate_random_vec() -> Vec<f64> {
    let mut rng = thread_rng(); 
    (0..39).map(|_| (rng.gen::<f64>().floor() * 5f64 ) - 5f64).collect::<Vec<f64>>()
}

fn generate_neighborhood(state: State) -> Neighborhood {
    let states = Vec::new();

    for i in 0..NEIGHBORHOOD_SIZE {
        let mut vec_tmp = vec![0f64; 0];
        for j in 0..state.vector.len() {
            vec_tmp[j] = state.vector[j] + (i as f64 * 0.1) as f64;
        }
    }

    Neighborhood { states }
}