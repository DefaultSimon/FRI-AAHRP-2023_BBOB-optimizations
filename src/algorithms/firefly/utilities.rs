#[derive(Clone)]
pub struct PointValue {
    pub position: Vec<f64>,
    pub value: f64,
}

impl PointValue {
    #[inline]
    pub fn new(position: Vec<f64>, value: f64) -> Self {
        Self { position, value }
    }
}
