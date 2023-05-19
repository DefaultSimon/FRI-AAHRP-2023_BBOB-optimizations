use std::fmt::{Display, Formatter, Write};
use std::path::Iter;
use crate::core::functions::BBOBFunction;

#[derive(Copy, Clone)]
pub struct SAOptions {
    // initial temperature
    pub function: BBOBFunction,
    pub initial_temperature: u8,

    // Cooling rate. Temperature is multiplied by this value on each iteration
    pub annealing_schedule: f64,
    pub max_iterations_sa: u64,
    pub max_iterations_ls: u64,
    pub initial_step_size_sa: f64,
    pub initial_step_size_ls: f64,
    pub min_temp: f64,
    pub n_best_sa: usize,
    pub n_best_ls: usize,
    pub seed: [u8; 16],
}

impl Default for SAOptions {
    fn default() -> Self {
        Self {
            function: BBOBFunction::AttractiveSector,
            initial_temperature: 100,
            annealing_schedule: 0.95,
            max_iterations_sa: 2000,
            max_iterations_ls: 10000,
            initial_step_size_sa: 0.01,
            initial_step_size_ls: 0.01,
            min_temp: 0.01,
            n_best_sa: 5,
            n_best_ls: 5,
            seed: [67, 193, 140, 181, 155, 182, 45, 146, 4, 213, 77, 160, 217, 31, 143, 135],
        }
    }
}

impl SAOptions {
    pub fn to_str(&self) -> String {
        format!("{{\
        \"initial_temperature: {},\n
        \"annealing_schedule: {},\n
        \"max_iterations_sa: {},\n
        \"max_iterations_ls: {},\n
        \"initial_step_size_sa: {},\n
        \"initial_step_size_ls: {},\n\
        \"min_temp: {},\n
        \"n_best_sa: {},\n
        \"n_best_ls: {},\n
        }}", self.initial_temperature, self.annealing_schedule, self.max_iterations_sa, self.max_iterations_ls, self.initial_step_size_sa, self.initial_step_size_ls,
                self.min_temp, self.n_best_sa, self.n_best_ls)
    }
}