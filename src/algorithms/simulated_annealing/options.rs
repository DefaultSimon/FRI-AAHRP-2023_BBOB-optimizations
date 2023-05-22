use crate::core::functions::BBOBFunctionType;

#[derive(Copy, Clone)]
pub struct SAOptions {
    // initial temperature
    pub function: BBOBFunctionType,
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
    pub ls_step_decrease: f64,
    pub seed: [u8; 16],
}

impl Default for SAOptions {
    fn default() -> Self {
        Self {
            function: BBOBFunctionType::AttractiveSector,
            initial_temperature: 100,
            annealing_schedule: 0.95,
            max_iterations_sa: 2000,
            max_iterations_ls: 1000,
            initial_step_size_sa: 0.05,
            initial_step_size_ls: 2f64,
            min_temp: 1f64,
            n_best_sa: 39,
            n_best_ls: 39,
            ls_step_decrease: 0.1,
            seed: [
                67, 193, 140, 181, 155, 182, 45, 146, 4, 213, 77, 160, 217, 31,
                143, 135,
            ],
        }
    }
}
