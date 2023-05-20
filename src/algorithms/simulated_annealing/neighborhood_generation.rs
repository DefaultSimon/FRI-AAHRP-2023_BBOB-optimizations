use itertools::Itertools;
use num::abs;

use crate::algorithms::common::rng::UniformF64BoundedRandomGenerator;
use crate::algorithms::common::structs::State;
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::core::problem::{BBOBProblem, Bounds};

pub struct LocalSearchNeighborhood {
    pub states: Vec<State>,
    rng: UniformF64BoundedRandomGenerator,
}

pub struct SANeighborhood {
    pub states: Vec<State>,
    rng: UniformF64BoundedRandomGenerator,
}

impl SANeighborhood {
    pub fn new(bounds: Bounds, seed: [u8; 16]) -> Self {
        let rng = UniformF64BoundedRandomGenerator::new(bounds, seed);
        Self {
            states: Vec::new(),
            rng,
        }
    }

    pub fn generate_neighborhood(
        &mut self,
        current_state: State,
        problem: &mut BBOBProblem,
        options: SAOptions,
    ) {
        let changes = self.find_biggest_change(
            current_state.clone().vector,
            problem,
            options.initial_step_size_sa,
        );
        self.states = Vec::new();
        for (i, el) in changes[0..options.n_best_ls].to_vec().iter().enumerate()
        {
            let mut new_state = current_state.clone().vector;
            if el.value_diff > 0f64 {
                if new_state[el.index] + options.initial_step_size_ls <= 5f64 {
                    new_state[el.index] += options.initial_step_size_ls;
                    self.states.push(State {
                        vector: new_state,
                        ..Default::default()
                    });
                }
            } else if new_state[el.index] - options.initial_step_size_ls >= -5f64
            {
                new_state[el.index] -= options.initial_step_size_ls;
                self.states.push(State {
                    vector: new_state,
                    ..Default::default()
                });
            }
        }
    }

    fn find_biggest_change(
        &mut self,
        current_neighborhood: Vec<f64>,
        problem: &mut BBOBProblem,
        step_size: f64,
    ) -> Vec<VectorElement> {
        let base_value = problem.evaluate(&current_neighborhood);

        let vec_elements: Vec<VectorElement> = (0..39)
            .map(|i| {
                let mut new_vec = current_neighborhood.clone();
                new_vec[i] += 0.01f64;
                let value = problem.evaluate(&new_vec);

                new_vec[i] -= 0.02f64;
                let value2 = problem.evaluate(&new_vec);
                let mut value_diff = 0f64;
                if base_value - value > base_value - value2 {
                    value_diff = abs(base_value - value);
                } else {
                    value_diff = -abs(base_value - value)
                }
                VectorElement::new(i, base_value - value)
            })
            .sorted_by(|el1, el2| el2.value_diff.total_cmp(&el1.value_diff))
            .collect();

        vec_elements
    }
}

impl LocalSearchNeighborhood {
    pub fn new(bounds: Bounds, seed: [u8; 16]) -> Self {
        let rng = UniformF64BoundedRandomGenerator::new(bounds, seed);
        Self {
            states: Vec::new(),
            rng,
        }
    }

    pub fn generate_neighborhood(
        &mut self,
        current_state: State,
        problem: &mut BBOBProblem,
        options: SAOptions,
    ) -> () {
        let changes = self.find_biggest_change(
            current_state.clone().vector,
            problem,
            options.initial_step_size_ls,
        );
        self.states = Vec::new();
        for (i, el) in changes[0..options.n_best_ls].iter().enumerate() {
            let mut new_state = current_state.clone().vector;
            if el.value_diff > 0f64 {
                if new_state[el.index] + options.initial_step_size_ls <= 5f64 {
                    new_state[el.index] += options.initial_step_size_ls;
                    self.states.push(State {
                        vector: new_state,
                        ..Default::default()
                    });
                }
            } else {
                if new_state[el.index] - options.initial_step_size_ls >= -5f64 {
                    new_state[el.index] -= options.initial_step_size_ls;
                    self.states.push(State {
                        vector: new_state,
                        ..Default::default()
                    });
                }
            }
        }
    }
    fn find_biggest_change(
        &mut self,
        current_neighborhood: Vec<f64>,
        problem: &mut BBOBProblem,
        step_size: f64,
    ) -> Vec<VectorElement> {
        let base_value = problem.evaluate(&current_neighborhood);

        let vec_elements: Vec<VectorElement> = (0..39)
            .map(|i| {
                let mut new_vec = current_neighborhood.clone();
                new_vec[i] += 0.01f64;
                let value = problem.evaluate(&new_vec);

                new_vec[i] -= 0.02f64;
                let value2 = problem.evaluate(&new_vec);
                let mut value_diff = 0f64;
                if base_value - value > base_value - value2 {
                    value_diff = abs(base_value - value);
                } else {
                    value_diff = -abs(base_value - value)
                }
                VectorElement::new(i, value_diff)
            })
            .sorted_by(|el1, el2| el2.value_diff.total_cmp(&el1.value_diff))
            .collect();

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
        Self { index, value_diff }
    }
}
