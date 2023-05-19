use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_pcg::{Mcg128Xsl64, Pcg64Mcg};

use crate::core::functions::BBOBFunctionType;

#[derive(Debug, Clone)]
pub struct FullFireflyOptions {
    /// Main random generator seed. This is used to generate other seeds
    /// used in various parts of the firefly algorithm.
    pub random_generator_seed: [u8; 16],

    /// Options for each run.
    pub per_restart_options: Vec<FireflyRunOptions>,

    pub post_process_best_options: Option<Vec<FireflyRunOptions>>,
}

/// References:
///  - [1: Firefly Algorithm: Recent Advances and Applications](https://arxiv.org/abs/1308.3898)
#[derive(Debug, Clone, Copy)]
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
        run_options,
        // Extremely high jitter variant:
        // - heats up very quickly early on when stuck and covers a very wide range when fully stuck,
        // - cools down very slowly,
        // - very patient as searching a wide area on high jitter might take quite a few iterations.
        FireflyRunOptions {
            consider_stuck_after_n_iterations: 2000,
            movement_jitter_starting_coefficient: 0.3,
            movement_jitter_cooling_factor: 0.9999,
            movement_jitter_min_stuck_runs_to_reheat: 120,
            movement_jitter_heating_factor: 1.05,
            movement_jitter_minimum_coefficient: 0.06,
            movement_jitter_maximum_coefficient: 8.0,
            ..run_options
        },
        // High jitter variant:
        // - heats up relatively quickly when stuck for a while, but searches a medium range,
        // - cools down slowly.
        FireflyRunOptions {
            consider_stuck_after_n_iterations: 1000,
            movement_jitter_starting_coefficient: 0.18,
            movement_jitter_cooling_factor: 0.999,
            movement_jitter_min_stuck_runs_to_reheat: 250,
            movement_jitter_heating_factor: 1.01,
            movement_jitter_minimum_coefficient: 0.05,
            movement_jitter_maximum_coefficient: 0.6,
            ..run_options
        },
        // Medium jitter variant:
        // - heats up slowly and not much, even when stuck for a long time
        //   (hoping to hit a better point locally by luck),
        // - cools down slowly.
        FireflyRunOptions {
            consider_stuck_after_n_iterations: 1000,
            movement_jitter_starting_coefficient: 0.1,
            movement_jitter_cooling_factor: 0.9995,
            movement_jitter_min_stuck_runs_to_reheat: 400,
            movement_jitter_heating_factor: 1.02,
            movement_jitter_minimum_coefficient: 0.02,
            movement_jitter_maximum_coefficient: 0.15,
            ..run_options
        },
        // Low jitter variant:
        // - barely heats up at all,
        // - cools relatively quickly,
        // - very patient and a lot of iterations.
        FireflyRunOptions {
            swarm_size: 50,
            maximum_iterations: run_options.maximum_iterations * 10,
            consider_stuck_after_n_iterations: 2000,
            movement_jitter_starting_coefficient: 0.005,
            movement_jitter_cooling_factor: 0.996,
            movement_jitter_min_stuck_runs_to_reheat: 800,
            movement_jitter_heating_factor: 1.0001,
            movement_jitter_minimum_coefficient: 0.0004,
            movement_jitter_maximum_coefficient: 0.01,
            ..run_options
        },
    ]
}

const DEFAULT_RNG_SEED: [u8; 16] = [
    133, 66, 79, 177, 132, 191, 158, 217, 101, 170, 134, 109, 79, 56, 2, 31,
];
const RNG_SEED_2: [u8; 16] = [
    68, 0, 111, 49, 202, 129, 188, 17, 242, 111, 237, 175, 192, 39, 186, 157,
];
const RNG_SEED_3: [u8; 16] = [
    91, 177, 196, 74, 108, 96, 138, 241, 113, 145, 26, 249, 86, 125, 133, 226,
];
const PREPROCESSING_RNG_SEED: [u8; 16] = [
    54, 243, 8, 243, 25, 11, 26, 252, 39, 239, 143, 200, 60, 100, 193, 63,
];

fn mutate_each_option_as_duplicate(
    runs: Vec<FireflyRunOptions>,
    preprocessing_seed: [u8; 16],
    mutation_range: Option<f64>,
) -> Vec<FireflyRunOptions> {
    let mutation_range = mutation_range.unwrap_or(0.05);

    let mut preprocessing_random_generator =
        Pcg64Mcg::from_seed(preprocessing_seed);
    let uniform_distribution =
        Uniform::new_inclusive(1f64 - mutation_range, 1f64 + mutation_range);

    let mutate_f64 = |value: f64, rng: &mut Mcg128Xsl64| {
        let coefficient = uniform_distribution.sample(rng);
        coefficient * value
    };
    let mutate_usize = |value: usize, rng: &mut Mcg128Xsl64| {
        let coefficient = uniform_distribution.sample(rng);
        ((value as f64) * coefficient).round() as usize
    };

    let runs_cloned: Vec<FireflyRunOptions> = runs
        .iter()
        .cloned()
        .map(|option| FireflyRunOptions {
            swarm_size: mutate_usize(
                option.swarm_size,
                &mut preprocessing_random_generator,
            ),
            maximum_iterations: mutate_usize(
                option.maximum_iterations,
                &mut preprocessing_random_generator,
            ),
            consider_stuck_after_n_iterations: mutate_usize(
                option.consider_stuck_after_n_iterations,
                &mut preprocessing_random_generator,
            ),
            attractiveness_coefficient: mutate_f64(
                option.attractiveness_coefficient,
                &mut preprocessing_random_generator,
            ),
            light_absorption_coefficient: mutate_f64(
                option.light_absorption_coefficient,
                &mut preprocessing_random_generator,
            ),
            movement_jitter_starting_coefficient: mutate_f64(
                option.movement_jitter_starting_coefficient,
                &mut preprocessing_random_generator,
            ),
            movement_jitter_cooling_factor: mutate_f64(
                option.movement_jitter_cooling_factor,
                &mut preprocessing_random_generator,
            )
            .min(1f64),
            movement_jitter_min_stuck_runs_to_reheat: mutate_usize(
                option.movement_jitter_min_stuck_runs_to_reheat,
                &mut preprocessing_random_generator,
            ),
            movement_jitter_heating_factor: mutate_f64(
                option.movement_jitter_heating_factor,
                &mut preprocessing_random_generator,
            )
            .max(1f64),
            movement_jitter_minimum_coefficient: mutate_f64(
                option.movement_jitter_minimum_coefficient,
                &mut preprocessing_random_generator,
            ),
            movement_jitter_maximum_coefficient: mutate_f64(
                option.movement_jitter_maximum_coefficient,
                &mut preprocessing_random_generator,
            ),
        })
        .collect();

    runs.into_iter().chain(runs_cloned).collect()
}


pub fn get_optimized_hyperparameters(
    problem: BBOBFunctionType,
) -> FullFireflyOptions {
    let base_restart_run = FireflyRunOptions {
        swarm_size: 80,
        maximum_iterations: 20000,
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

    let base_postprocessing_run_high_jitter = FireflyRunOptions {
        swarm_size: 100,
        maximum_iterations: 20000,
        consider_stuck_after_n_iterations: 1000,
        attractiveness_coefficient: 1f64,
        light_absorption_coefficient: 0.02,
        movement_jitter_starting_coefficient: 0.18,
        movement_jitter_cooling_factor: 0.999,
        movement_jitter_min_stuck_runs_to_reheat: 250,
        movement_jitter_heating_factor: 1.01,
        movement_jitter_minimum_coefficient: 0.05,
        movement_jitter_maximum_coefficient: 0.6,
    };

    let base_postprocessing_run_low_jitter = FireflyRunOptions {
        swarm_size: 100,
        maximum_iterations: 20000,
        consider_stuck_after_n_iterations: 2500,
        attractiveness_coefficient: 1f64,
        light_absorption_coefficient: 0.02,
        movement_jitter_starting_coefficient: 0.005,
        movement_jitter_cooling_factor: 0.996,
        movement_jitter_min_stuck_runs_to_reheat: 800,
        movement_jitter_heating_factor: 1.0001,
        movement_jitter_minimum_coefficient: 0.0002,
        movement_jitter_maximum_coefficient: 0.01,
    };

    let full_defaults = FullFireflyOptions {
        random_generator_seed: DEFAULT_RNG_SEED,
        per_restart_options: mutate_each_option_as_duplicate(
            generate_multiple_jitter_variants(base_restart_run),
            PREPROCESSING_RNG_SEED,
            Some(0.05),
        ),
        post_process_best_options: Some(vec![
            base_postprocessing_run_high_jitter,
            base_postprocessing_run_low_jitter,
        ]),
    };

    match problem {
        // <status> (delta=<distance to minimum>)
        // OK (delta=0.00000)
        BBOBFunctionType::Sphere => full_defaults,
        // NOT OK (delta=16.70670)
        BBOBFunctionType::SeparableEllipsoidal => FullFireflyOptions {
            random_generator_seed: DEFAULT_RNG_SEED,
            per_restart_options: generate_multiple_jitter_variants(
                FireflyRunOptions {
                    swarm_size: 40,
                    maximum_iterations: 20000,
                    consider_stuck_after_n_iterations: 1500,
                    attractiveness_coefficient: 1f64,
                    light_absorption_coefficient: 0.02,
                    movement_jitter_starting_coefficient: 0.065,
                    movement_jitter_cooling_factor: 0.985,
                    movement_jitter_min_stuck_runs_to_reheat: 300,
                    movement_jitter_heating_factor: 1.01,
                    movement_jitter_minimum_coefficient: 0.01,
                    movement_jitter_maximum_coefficient: 0.115,
                },
            ),
            post_process_best_options: Some(vec![
                base_postprocessing_run_high_jitter,
                base_postprocessing_run_low_jitter,
            ]),
        },
        // NOT OK (delta=516.37660)
        BBOBFunctionType::Rastrigin => full_defaults,
        // NOT OK (delta=550.76147)
        BBOBFunctionType::BucheRastrigin => full_defaults,
        // ALMOST OK (delta=6.50310)
        BBOBFunctionType::LinearSlope => full_defaults,
        // OK (delta=0.00003)
        BBOBFunctionType::AttractiveSector => full_defaults,
        // ALMOST OK (delta=11.28727)
        BBOBFunctionType::StepEllipsoidal => full_defaults,
        // OK (delta=0.01275)
        BBOBFunctionType::RosenbrockFunction => full_defaults,
        // OK (delta=0.47029)
        BBOBFunctionType::RosenbrockFunctionRotated => full_defaults,
        // ALMOST OK (delta=11.82058)
        BBOBFunctionType::Ellipsoidal => full_defaults,
        // OK (delta=0.00000)
        BBOBFunctionType::Discus => full_defaults,
        // OK (delta=0.21867)
        BBOBFunctionType::BentCigar => full_defaults,
        // ALMOST OK (delta=1.24980)
        BBOBFunctionType::SharpRidge => full_defaults,
        // OK (delta=0.00009)
        BBOBFunctionType::DifferentPowers => full_defaults,
        // NOT OK (delta=110.47083)
        // Heating helps a lot here.
        BBOBFunctionType::RastriginMultiModal => full_defaults,
        // ALMOST OK (delta=5.92256)
        BBOBFunctionType::Weierstrass => full_defaults,
        // ALMOST OK (delta=6.79358)
        BBOBFunctionType::SchafferF7 => full_defaults,
        // ALMOST OK (delta=26.02107)
        BBOBFunctionType::SchafferF7IllConditioned => full_defaults,
        // ALMOST OK (delta=1.75971)
        BBOBFunctionType::CompositeGriewankRosenbrockF8F2 => full_defaults,
        // ALMOST OK (delta=2.16887)
        BBOBFunctionType::Schwefel => full_defaults,
        // ALMOST OK (delta=0.69411)
        BBOBFunctionType::GallagherGaussian101MePeaks => full_defaults,
        // ALMOST OK (delta=2.59055)
        BBOBFunctionType::GallagherGaussian21HiPeaks => full_defaults,
        // OK (delta=0.27676)
        BBOBFunctionType::Katsuura => full_defaults,
        // NOT OK (delta=201.06055)
        BBOBFunctionType::LunacekBiRastrigin => FullFireflyOptions {
            random_generator_seed: RNG_SEED_3,
            per_restart_options: mutate_each_option_as_duplicate(
                generate_multiple_jitter_variants(FireflyRunOptions {
                    swarm_size: 150,
                    maximum_iterations: 10000,
                    consider_stuck_after_n_iterations: 800,
                    attractiveness_coefficient: 1f64,
                    light_absorption_coefficient: 0.001,
                    movement_jitter_starting_coefficient: 0.125,
                    movement_jitter_cooling_factor: 0.999,
                    movement_jitter_min_stuck_runs_to_reheat: 200,
                    movement_jitter_heating_factor: 1.008,
                    movement_jitter_minimum_coefficient: 0.009,
                    movement_jitter_maximum_coefficient: 0.6,
                }),
                PREPROCESSING_RNG_SEED,
                Some(0.09),
            ),
            post_process_best_options: Some(vec![
                base_postprocessing_run_high_jitter,
                base_postprocessing_run_low_jitter,
            ]),
        },
    }
}
