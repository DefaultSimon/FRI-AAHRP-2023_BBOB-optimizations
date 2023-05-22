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
    pub objective_value: f64,
}

impl State {
    pub fn set_objective_value(&mut self, obj_value: f64) -> () {
        self.objective_value = obj_value;
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            objective_value: f64::MAX,
            vector: Vec::new(),
        }
    }
}
