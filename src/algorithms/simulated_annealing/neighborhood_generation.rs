use coco_rs::LogLevel::Debug;
use crate::algorithms::common::structs::State;

pub struct LocalSearchNeighborhood {
    pub states: Vec<State>
}

pub struct SANeighborhood {
    pub states: Vec<State>
}

impl SANeighborhood {
    pub fn new() -> Self {
        Self {states: Vec::new()}
    }

    pub fn generate_neighborhood(&mut self, current_state: &State) -> () {
        self.states = Vec::new();
        self.states.push( State {vector: vec![0f64; 40], ..Default::default()});
    }
}
