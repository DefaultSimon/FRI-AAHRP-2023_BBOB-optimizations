#[derive(Debug, Clone)]
pub struct FullFireflyOptions {
    pub random_generator_seed: [u8; 16],

    /// How many restarts to do in one full firefly optimization.
    pub restart_count: usize,

    pub run_options: RunFireflyOptions,
}

impl Default for FullFireflyOptions {
    fn default() -> Self {
        Self {
            random_generator_seed: [
                133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79,
                56, 2, 31,
            ],
            restart_count: 4,
            run_options: RunFireflyOptions::default(),
        }
    }
}

/// References:
///  - [1: Firefly Algorithm: Recent Advances and Applications](https://arxiv.org/abs/1308.3898)
#[derive(Debug, Clone)]
pub struct RunFireflyOptions {
    /// Specified the amount of fireflies in the swarm. In FA, the swarm size is constant.
    /// According to [1], the optimal swarm size is between 15 to 100 (or 25 to 40).
    pub swarm_size: usize,

    /// A 16-byte random generator seed for the swarm initialization.
    // pub in_bounds_random_generator_seed: [u8; 16],

    /// A 16-byte seed for generating movement jitter.
    // pub jitter_movement_random_generator_seed: [u8; 16],

    /// Maximum of iterations to perform.
    pub maximum_iterations: usize,

    /// How many consequent iterations of non-improvement to tolerate before aborting the run
    /// (we probably got stuck in a local minimum) and returning the current minimum.
    pub consider_stuck_after_runs: usize,

    /// Coefficient of attraction to brighter fireflies (`β_0` in the paper [1]).
    /// Generally in range [0, 1] (0 being essentially random swarm search).
    pub attractiveness_coefficient: f64,

    /// Coefficient of light absorption (`γ` in the paper [1]).
    /// Generally in range [0, 1]. The smaller the value, the longer light travels
    /// (and the wider the "attraction field").
    pub light_absorption_coefficient: f64,

    /// To prevent getting stuck in local minimums, we add some jitter to firefly movements,
    /// this coefficient controls how much. The value is generally around `0.01 * problemSize`.
    // TODO Add simulated-annealing-like behaviour, see [1], page 2: 2.2 Parameter settings.
    pub movement_jitter_coefficient: f64,
}

impl Default for RunFireflyOptions {
    fn default() -> Self {
        Self {
            swarm_size: 150,
            maximum_iterations: 2000,
            consider_stuck_after_runs: 500,
            attractiveness_coefficient: 1f64,
            light_absorption_coefficient: 0.025,
            movement_jitter_coefficient: 0.01,
        }
    }
}
