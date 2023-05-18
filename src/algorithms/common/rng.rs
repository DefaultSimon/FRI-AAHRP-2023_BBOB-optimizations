use rand::distributions::{Distribution, Uniform};
use rand::{SeedableRng, thread_rng};
use rand::rngs::ThreadRng;
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

pub fn choose_random<T: Clone>(vec: Vec<T>) -> T {
    let mut rng = thread_rng();
    let uniform = Uniform::new(0, vec.len());
    vec[uniform.sample(&mut rng)].clone()
}

pub struct SimpleUniformRng {
    distribution: Uniform<f64>,
    rng: ThreadRng
}

impl SimpleUniformRng {
    pub fn new(low: f64, high: f64) -> Self {
        let distribution = Uniform::new(low, high);
        let rng = thread_rng();
        Self {
            distribution, rng
        }
    }

    pub fn sample(&mut self) -> f64 {
        self.distribution.sample(&mut self.rng)
    }

    pub fn sample_multiple(&mut self, size: usize) -> Vec<f64> {
        (0..size).map(|_| self.sample()).collect()
    }
}
