#[derive(Debug, Clone)]
pub struct FullFireflyOptions {
    /// Main random generator seed. This is used to generate other seeds
    /// used in various parts of the firefly algorithm.
    pub random_generator_seed: [u8; 16],

    /// How many restarts to do in one full firefly optimization.
    pub restart_count: usize,

    /// Run-specific options, like the iteration count and the swarm size.
    pub run_options: FireflyRunOptions,
}

impl Default for FullFireflyOptions {
    fn default() -> Self {
        Self {
            random_generator_seed: [
                133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79,
                56, 2, 31,
            ],
            restart_count: 4,
            run_options: FireflyRunOptions::default(),
        }
    }
}

/// References:
///  - [1: Firefly Algorithm: Recent Advances and Applications](https://arxiv.org/abs/1308.3898)
#[derive(Debug, Clone)]
pub struct FireflyRunOptions {
    /// Specified the amount of fireflies in the swarm. In FA, the swarm size is constant.
    /// According to [1], the optimal swarm size is between 15 to 100 (or 25 to 40).
    pub swarm_size: usize,

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
    pub movement_jitter_starting_coefficient: f64,

    /// Lower bound for the movement jitter coefficient.
    pub movement_jitter_minimum_coefficient: f64,

    /// Cooling factor associated with the movement jitter coefficient.
    /// A value of `0.95` means the jitter decreases by that factor each iteration.
    /// A value of `1` effectively means no jitter decrease is applied.
    pub movement_jitter_cooling_factor: f64,
}

impl Default for FireflyRunOptions {
    fn default() -> Self {
        Self {
            swarm_size: 150,
            maximum_iterations: 2000,
            consider_stuck_after_runs: 500,
            attractiveness_coefficient: 1f64,
            light_absorption_coefficient: 0.025,
            movement_jitter_starting_coefficient: 0.01,
            movement_jitter_minimum_coefficient: 0.005,
            movement_jitter_cooling_factor: 0.99,
        }
    }
}
