use std::{ops::Add, ptr::null, f64::consts::E};
use std::ops::Sub;

use crate::{core::problem::BBOBProblem, algorithms::common::rng::UniformRNG};
use super::super::common::structs::Minimum;
use miette::{Result, miette};
use rand::{rngs::*, thread_rng, Rng};
use rand_pcg::Pcg64Mcg;
use crate::algorithms::firefly::FireflyOptions;
use crate::algorithms::simulated_annealing::options::SAOptions;

static NEIGHBORHOOD_SIZE: u64 = 100;

pub fn run_sa(mut problem: BBOBProblem, options: SAOptions) -> Result<Minimum> {
    let mut rng = UniformRNG::new(problem.bounds(), [1,2,3,4,5,1,2,3,4,5,1,2,3,4,1,3]);
    let mut current_state = State { vector:  rng.sample_multiple(40)};
    let mut minimal_state = current_state.clone();
    let mut minimal_value = problem.evaluate(&current_state.vector);
    let neighborhood = generate_neighborhood(current_state);
    let mut temperature = options.temperature as f64;

    while temperature > 0.1 {
        let next_state = random_choice(&neighborhood);
        let result = problem.evaluate(&next_state.vector);

        if result < minimal_value {
            minimal_state = next_state;
            minimal_value = result;
        } else {
            let p_move = E.powf((result - minimal_value) / temperature);
            let mut rng = thread_rng();
            if rng.gen::<f64>() <= p_move {
                current_state = next_state;
            }
            temperature = options.annealing_schedule * temperature;
        }
    }
    local_search(problem, minimal_state)
}

fn local_search(mut problem: BBOBProblem, start_state: State) -> Result<Minimum> {
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

fn random_choice(neighborhood: &Neighborhood) -> State {
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
        let mut vec_tmp = vec![0f64; 40];
        for (j, el) in state.vector.iter().enumerate() {
            vec_tmp[j] = state.vector[j] + (i as f64 * 0.1) as f64;
        }
    }

    Neighborhood { states }
}