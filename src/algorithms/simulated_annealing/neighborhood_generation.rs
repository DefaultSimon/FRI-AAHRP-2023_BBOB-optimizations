use std::ffi::c_char;

use coco_rs::LogLevel::Debug;
use itertools::Itertools;

use crate::algorithms::common::rng::{SimpleUniformRng, UniformF64BoundedRandomGenerator};
use crate::algorithms::common::structs::State;
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::core::problem::{BBOBProblem, Bounds};

pub struct LocalSearchNeighborhood {
    pub states: Vec<State>,
    rng: UniformF64BoundedRandomGenerator
}

pub struct SANeighborhood {
    pub states: Vec<State>,
    rng: UniformF64BoundedRandomGenerator
}

impl SANeighborhood {
    pub fn new(bounds: Bounds, seed: [u8;16]) -> Self {
        let rng = UniformF64BoundedRandomGenerator::new(bounds, seed);
        Self { states: Vec::new(), rng }
    }

    pub fn generate_neighborhood(&mut self, current_state: State, problem: &mut BBOBProblem, options: SAOptions) -> () {
        let changes = self.find_biggest_change(current_state.clone().vector, problem);
        self.states = Vec::new();
        for (i, el) in changes[0..options.n_best_ls].to_vec().iter().enumerate() {
            let mut new_state = current_state.clone().vector;
            if new_state[el.index] + options.initial_step_size_sa <= 5f64 {
                new_state[el.index] += options.initial_step_size_sa * (1f64 / i as f64);
                self.states.push(State { vector: new_state, ..Default::default() });
            }

            new_state = current_state.clone().vector;
            if new_state[el.index] + options.initial_step_size_sa >= -5f64 {
                new_state[el.index] -= options.initial_step_size_sa * (1f64 / i as f64);
                self.states.push(State { vector: new_state, ..Default::default() });
            }
        }
    }

    fn find_biggest_change(&mut self, current_neighborhood: Vec<f64>, problem: &mut BBOBProblem) -> Vec<VectorElement> {
        let base_value = problem.evaluate(&current_neighborhood);

        let vec_elements: Vec<VectorElement> = (0..39).map(|i| {
            let mut new_vec = current_neighborhood.clone();
            new_vec[i] += 0.01f64;
            let value = problem.evaluate(&new_vec);

            VectorElement::new(i, base_value - value)
        }).sorted_by(|el1, el2| el2.value_diff.total_cmp(&el1.value_diff)).collect();

        vec_elements
    }
}

impl LocalSearchNeighborhood {
    pub fn new(bounds: Bounds, seed: [u8;16]) -> Self {
        let rng = UniformF64BoundedRandomGenerator::new(bounds, seed);
        Self { states: Vec::new(), rng }
    }

    pub fn generate_neighborhood(&mut self, current_state: State, problem: &mut BBOBProblem, options: SAOptions) -> () {
        let changes = self.find_biggest_change(current_state.clone().vector, problem);
        self.states = Vec::new();
        for (i, el) in changes[0..options.n_best_ls].iter().enumerate() {
            let mut new_state = current_state.clone().vector;
            if new_state[el.index] + options.initial_step_size_ls <= 5f64 {
                new_state[el.index] += options.initial_step_size_ls * (1f64 / i as f64);
                self.states.push(State { vector: new_state, ..Default::default() });
            }

            new_state = current_state.clone().vector;
            if new_state[el.index] - options.initial_step_size_ls >= -5f64 {
                new_state[el.index] -= options.initial_step_size_ls * (1f64 / i as f64);
                self.states.push(State { vector: new_state, ..Default::default() });
            }
        }
    }
    fn find_biggest_change(&mut self, current_neighborhood: Vec<f64>, problem: &mut BBOBProblem) -> Vec<VectorElement> {
        let base_value = problem.evaluate(&current_neighborhood);

        let vec_elements: Vec<VectorElement> = (0..39).map(|i| {
            let mut new_vec = current_neighborhood.clone();
            new_vec[i] += 0.01f64;
            let value = problem.evaluate(&new_vec);

            VectorElement::new(i, base_value - value)
        }).sorted_by(|el1, el2| el2.value_diff.total_cmp(&el1.value_diff)).collect();

        vec_elements
    }
}

#[derive(Copy, Clone)]
struct VectorElement {
    pub index: usize,
    pub value_diff: f64,
}

impl VectorElement {
    pub fn new(index: usize, value_diff: f64) -> Self {
        Self {
            index,
            value_diff,
        }
    }
}