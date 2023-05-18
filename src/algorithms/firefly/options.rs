use crate::core::functions::BBOBFunctionType;

#[derive(Debug, Clone)]
pub struct FullFireflyOptions {
    /// Main random generator seed. This is used to generate other seeds
    /// used in various parts of the firefly algorithm.
    pub random_generator_seed: [u8; 16],

    /// Options per-restart of each
    pub per_restart_options: Vec<FireflyRunOptions>,
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


pub fn get_optimized_hyperparameters(
    problem: BBOBFunctionType,
) -> FullFireflyOptions {
    let default_run_options = FireflyRunOptions {
        swarm_size: 80,
        maximum_iterations: 2000,
        consider_stuck_after_runs: 500,
        attractiveness_coefficient: 1f64,
        light_absorption_coefficient: 0.025,
        movement_jitter_starting_coefficient: 0.1,
        movement_jitter_minimum_coefficient: 0.005,
        movement_jitter_cooling_factor: 0.98,
    };

    let defaults = FullFireflyOptions {
        random_generator_seed: [
            133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79, 56, 2,
            31,
        ],
        per_restart_options: vec![
            default_run_options.clone(),
            FireflyRunOptions {
                movement_jitter_starting_coefficient: 0.5,
                movement_jitter_minimum_coefficient: 0.08,
                movement_jitter_cooling_factor: 0.9999,
                ..default_run_options
            },
            FireflyRunOptions {
                movement_jitter_starting_coefficient: 0.001,
                movement_jitter_minimum_coefficient: 0.0001,
                movement_jitter_cooling_factor: 0.95,
                ..default_run_options
            },
        ],
    };

    match problem {
        // OK (delta=0.00005)
        BBOBFunctionType::Sphere => defaults,
        // NOT OK
        BBOBFunctionType::SeparableEllipsoidal => defaults,
        // NOT OK
        BBOBFunctionType::Rastrigin => defaults,
        // NOT OK
        BBOBFunctionType::BucheRastrigin => defaults,
        // NOT OK
        BBOBFunctionType::LinearSlope => defaults,
        // NOT OK
        BBOBFunctionType::AttractiveSector => defaults,
        // NOT OK
        BBOBFunctionType::StepEllipsoidal => defaults,
        // NOT OK
        BBOBFunctionType::RosenbrockFunction => defaults,
        // NEARLY THERE (delta=5.27988)
        BBOBFunctionType::RosenbrockFunctionRotated => defaults,
        // NOT OK
        BBOBFunctionType::Ellipsoidal => defaults,
        // NEARLY THERE (delta=25.36596)
        BBOBFunctionType::Discus => defaults,
        // NOT OK
        BBOBFunctionType::BentCigar => defaults,
        // NOT OK
        BBOBFunctionType::SharpRidge => defaults,
        // OK (delta=0.00107)
        BBOBFunctionType::DifferentPowers => defaults,
        // NOT OK
        BBOBFunctionType::RastriginMultiModal => defaults,
        // NOT OK
        BBOBFunctionType::Weierstrass => defaults,
        // NOT OK
        BBOBFunctionType::SchafferF7 => defaults,
        // NOT OK
        BBOBFunctionType::SchafferF7IllConditioned => defaults,
        // NOT OK
        BBOBFunctionType::CompositeGriewankRosenbrockF8F2 => defaults,
        // NOT OK
        BBOBFunctionType::Schwefel => defaults,
        // NOT OK
        BBOBFunctionType::GallagherGaussian101MePeaks => defaults,
        // NOT OK
        BBOBFunctionType::GallagherGaussian21HiPeaks => defaults,
        // NOT OK
        BBOBFunctionType::Katsuura => defaults,
        // NOT OK
        BBOBFunctionType::LunacekBiRastrigin => defaults,
    }
}
