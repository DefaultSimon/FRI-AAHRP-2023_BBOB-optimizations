#[derive(Copy, Clone)]
pub struct SAOptions {
    // initial temperature
    pub temperature: u8,

    // Cooling rate. Temperature is multiplied by this value on each iteration
    pub annealing_schedule: f64,
    pub max_iterations: u16
}

impl Default for SAOptions {
    fn default() -> Self {
        Self {
            temperature: 100,
            annealing_schedule: 0.95,
            max_iterations: 2000
        }
    }
}