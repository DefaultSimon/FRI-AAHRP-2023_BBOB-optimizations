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
