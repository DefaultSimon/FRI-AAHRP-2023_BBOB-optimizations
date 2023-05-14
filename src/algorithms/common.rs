use rand::{rngs::*, thread_rng, Rng};

#[derive(Debug)]
pub struct Minimum<'a> {
    value: f64,
    vector: &'a [f64]
}

impl<'a> Minimum<'a> {
    fn new(value: f64, vector: &'a [f64]) -> Minimum {
        Self { value, vector }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State<'a> {
    vector: &'a [f64]
}

#[derive(Debug)]
pub struct Neighborhood<'a> {
    states: &'a [State<'a>]
}

pub fn random_choice<'a>(neighborhood: &'a Neighborhood) -> State<'a> {
    let mut rng = thread_rng();
    let rand_num = rng.gen_range::<usize, _>(0..neighborhood.states.len());
    neighborhood.states[rand_num]
}