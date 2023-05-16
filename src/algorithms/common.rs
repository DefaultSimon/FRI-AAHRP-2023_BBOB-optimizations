use std::ops::Mul;

use rand::{rngs::*, thread_rng, Rng};

#[derive(Debug)]
pub struct Minimum {
    pub value: f64,
    pub vector: Vec<f64>,
}

impl Minimum {
    #[inline]
    pub fn new(value: f64, vector: Vec<f64>) -> Minimum {
        Self { value, vector }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub vector: Vec<f64>,
}

#[derive(Debug)]
pub struct Neighborhood {
    pub states: Vec<State>,
}

pub fn random_choice<'a>(neighborhood: &'a Neighborhood) -> State {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range::<usize, _>(0..neighborhood.states.len());
    neighborhood.states[rand_num].clone()
}

pub fn generate_random_vec() -> Vec<f64> {
    let mut rng = thread_rng();
    (0..39)
        .map(|_| (rng.gen::<f64>().floor() * 5f64) - 5f64)
        .collect::<Vec<f64>>()
}
