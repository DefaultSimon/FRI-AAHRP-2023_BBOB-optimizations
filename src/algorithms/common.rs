#[derive(Debug)]
pub struct Minimum {
    value: f64,
    vector: Vec<f64>
}

impl Minimum {
    fn new(value: f64, vector: Vec<f64>) -> Minimum {
        Minimum { value, vector }
    }
}