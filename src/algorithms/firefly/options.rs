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
    pub consider_stuck_after_n_iterations: usize,

    /// Coefficient of attraction to brighter fireflies (`β_0` in the paper [1]).
    /// Generally in range [0, 1] (0 being essentially random swarm search).
    pub attractiveness_coefficient: f64,

    /// Coefficient of light absorption (`γ` in the paper [1]).
    /// Generally in range [0, 1]. The smaller the value, the longer light travels
    /// (and the wider the "attraction field").
    pub light_absorption_coefficient: f64,

    /// To prevent getting stuck in local minimums, we add some jitter to firefly movements,
    /// this coefficient controls how much we start with.
    /// A good starting value is generally around `0.01 * problem_size`.
    pub movement_jitter_starting_coefficient: f64,

    /// Cooling factor associated with the movement jitter coefficient.
    /// A value of `0.95` means the jitter decreases by that factor each iteration.
    /// A value of `1` effectively means no jitter decrease is applied.
    pub movement_jitter_cooling_factor: f64,

    /// We generally cool the jitter coefficient down as the iterations progress,
    /// but if we get stuck in a minimum we can't get out of (i.e. we're stuck for several iterations),
    /// we might want to start heating the jitter back up to escape.
    /// This parameter controls how many stuck iterations need to happen for the jitter to start heating up.
    pub movement_jitter_min_stuck_runs_to_reheat: usize,

    /// Heating factor associated with the movement jitter coefficient.
    /// A value of `2` (which is absolutely too high) means the jitter will double.
    pub movement_jitter_heating_factor: f64,

    /// Lower bound for the movement jitter coefficient, meaning the cooling factor
    /// cannot cool down the jitter more than this.
    pub movement_jitter_minimum_coefficient: f64,

    /// Upper bound for the movement jitter coefficient, meaning the heating factor
    /// cannot heat the jitter more than this.
    pub movement_jitter_maximum_coefficient: f64,
}

#[allow(dead_code)]
impl FireflyRunOptions {
    pub fn with_swarm_size(self, swarm_size: usize) -> Self {
        Self { swarm_size, ..self }
    }

    pub fn with_maximum_iterations(self, maximum_iterations: usize) -> Self {
        Self {
            maximum_iterations,
            ..self
        }
    }

    pub fn with_consider_stuck_after_runs(
        self,
        consider_stuck_after_runs: usize,
    ) -> Self {
        Self {
            consider_stuck_after_n_iterations: consider_stuck_after_runs,
            ..self
        }
    }

    pub fn with_attractiveness_coefficient(
        self,
        attractiveness_coefficient: f64,
    ) -> Self {
        Self {
            attractiveness_coefficient,
            ..self
        }
    }

    pub fn with_light_absorption_coefficient(
        self,
        light_absorption_coefficient: f64,
    ) -> Self {
        Self {
            light_absorption_coefficient,
            ..self
        }
    }

    pub fn with_movement_jitter_starting_coefficient(
        self,
        movement_jitter_starting_coefficient: f64,
    ) -> Self {
        Self {
            movement_jitter_starting_coefficient,
            ..self
        }
    }

    pub fn with_movement_jitter_cooling_factor(
        self,
        movement_jitter_cooling_factor: f64,
    ) -> Self {
        Self {
            movement_jitter_cooling_factor,
            ..self
        }
    }

    pub fn with_movement_jitter_min_stuck_runs_to_reheat(
        self,
        movement_jitter_min_stuck_runs_to_reheat: usize,
    ) -> Self {
        Self {
            movement_jitter_min_stuck_runs_to_reheat,
            ..self
        }
    }

    pub fn with_movement_jitter_heating_factor(
        self,
        movement_jitter_heating_factor: f64,
    ) -> Self {
        Self {
            movement_jitter_heating_factor,
            ..self
        }
    }

    pub fn with_movement_jitter_minimum_coefficient(
        self,
        movement_jitter_minimum_coefficient: f64,
    ) -> Self {
        Self {
            movement_jitter_minimum_coefficient,
            ..self
        }
    }

    pub fn with_movement_jitter_maximum_coefficient(
        self,
        movement_jitter_maximum_coefficient: f64,
    ) -> Self {
        Self {
            movement_jitter_maximum_coefficient,
            ..self
        }
    }
}

impl Default for FireflyRunOptions {
    fn default() -> Self {
        Self {
            swarm_size: 150,
            maximum_iterations: 2000,
            consider_stuck_after_n_iterations: 500,
            attractiveness_coefficient: 1f64,
            light_absorption_coefficient: 0.025,
            movement_jitter_starting_coefficient: 0.01,
            movement_jitter_cooling_factor: 0.99,
            movement_jitter_min_stuck_runs_to_reheat: 200,
            movement_jitter_heating_factor: 1.1,
            movement_jitter_minimum_coefficient: 0.005,
            movement_jitter_maximum_coefficient: 0.5,
        }
    }
}

fn generate_multiple_jitter_variants(
    run_options: FireflyRunOptions,
) -> Vec<FireflyRunOptions> {
    vec![
        // Original untouched options.
        run_options.clone(),
        // Extremely high jitter variant that heats up very quickly when stuck and cools down very slowly.
        FireflyRunOptions {
            movement_jitter_starting_coefficient: 0.22,
            movement_jitter_cooling_factor: 0.9999,
            movement_jitter_min_stuck_runs_to_reheat: 100,
            movement_jitter_heating_factor: 1.15,
            movement_jitter_minimum_coefficient: 0.06,
            movement_jitter_maximum_coefficient: 0.8,
            ..run_options
        },
        // High jitter variant that heats up very quickly when stuck and cools down slowly.
        FireflyRunOptions {
            movement_jitter_starting_coefficient: 0.18,
            movement_jitter_cooling_factor: 0.999,
            movement_jitter_min_stuck_runs_to_reheat: 250,
            movement_jitter_heating_factor: 1.15,
            movement_jitter_minimum_coefficient: 0.06,
            movement_jitter_maximum_coefficient: 0.4,
            ..run_options
        },
        // Medium jitter variant. Cools down slowly, heats up slowly.
        FireflyRunOptions {
            movement_jitter_starting_coefficient: 0.1,
            movement_jitter_cooling_factor: 0.99,
            movement_jitter_min_stuck_runs_to_reheat: 400,
            movement_jitter_heating_factor: 1.02,
            movement_jitter_minimum_coefficient: 0.02,
            movement_jitter_maximum_coefficient: 0.15,
            ..run_options
        },
        // Low jitter variant, cools relatively quickly, barely heats up.
        FireflyRunOptions {
            movement_jitter_starting_coefficient: 0.005,
            movement_jitter_cooling_factor: 0.97,
            movement_jitter_min_stuck_runs_to_reheat: 800,
            movement_jitter_heating_factor: 1.005,
            movement_jitter_minimum_coefficient: 0.0002,
            movement_jitter_maximum_coefficient: 0.01,
            ..run_options
        },
    ]
}


pub fn get_optimized_hyperparameters(
    problem: BBOBFunctionType,
) -> FullFireflyOptions {
    const DEFAULT_RNG_SEED: [u8; 16] = [
        133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79, 56, 2, 31,
    ];

    let base_run_options = FireflyRunOptions {
        swarm_size: 80,
        maximum_iterations: 15000,
        consider_stuck_after_n_iterations: 500,
        attractiveness_coefficient: 1f64,
        light_absorption_coefficient: 0.02,
        movement_jitter_starting_coefficient: 0.065,
        movement_jitter_cooling_factor: 0.985,
        movement_jitter_min_stuck_runs_to_reheat: 300,
        movement_jitter_heating_factor: 1.01,
        movement_jitter_minimum_coefficient: 0.005,
        movement_jitter_maximum_coefficient: 0.115,
    };

    let with_jitter_variants = |run_base| FullFireflyOptions {
        random_generator_seed: DEFAULT_RNG_SEED,
        per_restart_options: generate_multiple_jitter_variants(run_base),
    };

    match problem {
        // <status> (delta=<distance to minimum>)
        // OK (delta=0.00006)
        BBOBFunctionType::Sphere => with_jitter_variants(base_run_options),
        // NOT OK (delta=603.90328)
        BBOBFunctionType::SeparableEllipsoidal => with_jitter_variants(
            base_run_options
                .with_swarm_size(40)
                .with_maximum_iterations(20000)
                .with_movement_jitter_minimum_coefficient(0.01)
                .with_consider_stuck_after_runs(1500),
        ),
        // NOT OK (delta=516.37685)
        BBOBFunctionType::Rastrigin => with_jitter_variants(
            base_run_options
                .with_swarm_size(50)
                .with_maximum_iterations(10000)
                .with_consider_stuck_after_runs(1000)
                .with_movement_jitter_min_stuck_runs_to_reheat(150)
                .with_light_absorption_coefficient(0.009),
        ),
        // NOT OK (delta=659.69163)
        BBOBFunctionType::BucheRastrigin => {
            with_jitter_variants(base_run_options)
        }
        // ALMOST OK (delta=6.64265)
        BBOBFunctionType::LinearSlope => with_jitter_variants(base_run_options),
        // OK (delta=0.00251)
        BBOBFunctionType::AttractiveSector => {
            with_jitter_variants(base_run_options)
        }
        // ALMOST OK (delta=11.45838)
        BBOBFunctionType::StepEllipsoidal => {
            with_jitter_variants(base_run_options)
        }
        // OK (delta=0.70861)
        BBOBFunctionType::RosenbrockFunction => {
            with_jitter_variants(base_run_options)
        }
        // OK (delta=0.58336)
        BBOBFunctionType::RosenbrockFunctionRotated => {
            with_jitter_variants(base_run_options)
        }
        // NOT OK (delta=192.53581)
        BBOBFunctionType::Ellipsoidal => with_jitter_variants(base_run_options),
        // OK (delta=0.00014)
        BBOBFunctionType::Discus => with_jitter_variants(base_run_options),
        // NOT OK (delta=42.96076)
        BBOBFunctionType::BentCigar => with_jitter_variants(base_run_options),
        // ALMOST OK (delta=1.24980)
        BBOBFunctionType::SharpRidge => with_jitter_variants(base_run_options),
        // OK (delta=0.00068)
        BBOBFunctionType::DifferentPowers => {
            with_jitter_variants(base_run_options)
        }
        // NOT OK (delta=342.98268)
        BBOBFunctionType::RastriginMultiModal => {
            with_jitter_variants(base_run_options)
        }
        // ALMOST OK (delta=9.48454)
        BBOBFunctionType::Weierstrass => with_jitter_variants(base_run_options),
        // ALMOST OK (delta=6.36824)
        BBOBFunctionType::SchafferF7 => with_jitter_variants(base_run_options),
        // ALMOST OK (delta=6.75608)
        BBOBFunctionType::SchafferF7IllConditioned => {
            with_jitter_variants(base_run_options)
        }
        // ALMOST OK (delta=2.22957)
        BBOBFunctionType::CompositeGriewankRosenbrockF8F2 => {
            with_jitter_variants(base_run_options)
        }
        // ALMOST OK (delta=2.27812)
        BBOBFunctionType::Schwefel => with_jitter_variants(base_run_options),
        // ALMOST OK (delta=1.93775)
        BBOBFunctionType::GallagherGaussian101MePeaks => {
            with_jitter_variants(base_run_options)
        }
        // ALMOST OK (delta=2.59057)
        BBOBFunctionType::GallagherGaussian21HiPeaks => {
            with_jitter_variants(base_run_options)
        }
        // OK (delta=0.40514)
        BBOBFunctionType::Katsuura => with_jitter_variants(base_run_options),
        // NOT OK (delta=190.74857)
        BBOBFunctionType::LunacekBiRastrigin => {
            with_jitter_variants(base_run_options)
        }
    }
}
