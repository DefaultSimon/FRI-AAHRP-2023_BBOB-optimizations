use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

use crate::core::problem::Bounds;

#[derive(Clone)]
pub struct UniformRNG {
    pub bounds: Bounds,

    distribution: Uniform<f64>,
    rng: Pcg64Mcg,
}

impl UniformRNG {
    pub fn new(bounds: Bounds, seed: [u8; 16]) -> Self {
        let distribution = bounds.uniform_random_generator();
        let pcg_rng = Pcg64Mcg::from_seed(seed);

        Self {
            bounds,
            distribution,
            rng: pcg_rng,
        }
    }

    pub fn sample(&mut self) -> f64 {
        self.distribution.sample(&mut self.rng)
    }

    pub fn sample_multiple(&mut self, samples: usize) -> Vec<f64> {
        (0..samples)
            .map(|_| self.distribution.sample(&mut self.rng))
            .collect()
    }
}
