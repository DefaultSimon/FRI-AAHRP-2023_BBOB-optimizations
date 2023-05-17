use std::f64::consts::E;
use std::iter::zip;

use num::clamp;

use crate::algorithms::firefly::options::FireflyRunOptions;
use crate::algorithms::firefly::rng::UniformF64BoundedRandomGenerator;
use crate::core::problem::BBOBProblem;

/// Individual firefly in the swarm.
#[derive(Clone)]
pub struct Firefly {
    pub position: Vec<f64>,
    pub objective_function_value: f64,
}

impl Firefly {
    pub fn new(position: Vec<f64>, problem: &mut BBOBProblem) -> Self {
        assert_eq!(
            problem.input_dimensions,
            position.len(),
            "Input dimensions did not match!"
        );

        let objective_function_value = problem.evaluate(&position);
        Self {
            position,
            objective_function_value,
        }
    }

    pub fn move_towards(
        &mut self,
        second_firefly: &Firefly,
        problem: &mut BBOBProblem,
        minus_half_to_half_uniform_generator: &mut UniformF64BoundedRandomGenerator,
        options: &FireflyRunOptions,
    ) {
        // Calculate attraction coefficient (essentially how much the firefly will move towards the `other_firefly`).
        let euclidean_distance_squared = self
            .position
            .iter()
            .zip(second_firefly.position.iter())
            .map(|(first, second)| {
                let difference = *first - *second;
                difference * difference
            })
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
            let updated_value = *our_value
                // Move towards the brighter firefly by the attraction coefficient.
                + attraction_coefficient * (*other_firefly_value - *our_value)
                // Add some random jitter, uniformly sampled and multiplied by the jitter coefficient.
                + options.movement_jitter_coefficient
                    * minus_half_to_half_uniform_generator.sample();

            clamp(updated_value, -5f64, 5f64)
        })
        .collect();

        self.position = final_position;
        self.objective_function_value = problem.evaluate(&self.position);
    }
}
