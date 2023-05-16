use std::f64::consts::E;
use std::iter::zip;

use crate::algorithms::firefly::options::FireflyOptions;
use crate::algorithms::firefly::rng::UniformRNG;
use crate::core::problem::BBOBProblem;

/// Individual firefly in the swarm.
#[derive(Clone)]
pub struct Firefly {
    pub uniform_zero_to_one_generator: UniformRNG,
    pub position: Vec<f64>,
    pub objective_function_value: f64,
}

impl Firefly {
    pub fn new(
        uniform_zero_to_one_generator: UniformRNG,
        position: Vec<f64>,
        problem: &BBOBProblem,
    ) -> Self {
        assert_eq!(
            problem.input_dimensions,
            position.len(),
            "Input dimensions did not match!"
        );

        let objective_function_value = problem.evaluate(&position);
        Self {
            uniform_zero_to_one_generator,
            position,
            objective_function_value,
        }
    }

    pub fn move_towards(
        &mut self,
        second_firefly: &Firefly,
        problem: &BBOBProblem,
        options: &FireflyOptions,
    ) {
        // Calculate attraction coefficient (essentially how much the firefly will move towards the `other_firefly`).
        let euclidean_distance_squared = self
            .position
            .iter()
            .zip(second_firefly.position.iter())
            .map(|(first, second)| (*first - *second).powi(2))
            .sum::<f64>();

        let attraction_coefficient = options.attractiveness_coefficient
            * E.powf(
                -1f64
                    * options.light_absorption_coefficient
                    * euclidean_distance_squared,
            );


        // Calculate the final value in each dimension.
        let final_position: Vec<f64> = zip(
            self.position.iter(),
            second_firefly.position.iter(),
        )
        .map(|(our_value, other_firefly_value)| {
            *our_value
                // Move towards the brighter firefly by the attraction coefficient.
                + attraction_coefficient * (*other_firefly_value - *our_value)
                // Add some random jitter, uniformly sampled and multiplied by the jitter coefficient.
                + options.movement_jitter_coefficient
                    * (self.uniform_zero_to_one_generator.sample() - 0.5f64)
        })
        .collect();

        self.position = final_position;
        self.objective_function_value = problem.evaluate(&self.position);
    }
}
