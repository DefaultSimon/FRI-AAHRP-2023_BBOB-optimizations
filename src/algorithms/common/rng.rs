use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;
use rand_pcg::Pcg64Mcg;

use crate::core::problem::Bounds;

#[derive(Clone)]
pub struct UniformU8RandomGenerator {
    distribution: Uniform<u8>,
    rng: Pcg64Mcg,
}

impl UniformU8RandomGenerator {
    pub fn new(seed: [u8; 16]) -> Self {
        let distribution = Uniform::new_inclusive(u8::MIN, u8::MAX);
        let pcg_rng = Pcg64Mcg::from_seed(seed);

        Self {
            distribution,
            rng: pcg_rng,
        }
    }

    pub fn sample_multiple<const L: usize>(&mut self) -> [u8; L] {
        (0..L)
            .map(|_| self.distribution.sample(&mut self.rng))
            .collect::<Vec<u8>>()
            .try_into()
            .expect("BUG: Could not collect into array of size L.")
    }
}



#[derive(Clone)]
pub struct UniformF64BoundedRandomGenerator {
    pub bounds: Bounds,

    distribution: Uniform<f64>,
    rng: Pcg64Mcg,
}

impl UniformF64BoundedRandomGenerator {
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
