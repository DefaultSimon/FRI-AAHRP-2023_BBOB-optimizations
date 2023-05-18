use crate::algorithms::common::structs::Minimum;
use crate::algorithms::firefly::utilities::PointValue;

pub struct FireflyFullOptimizationResult {
    // Iterations performed per-restart.
    pub iterations_performed_per_restart: Vec<usize>,

    // Minimum found throughout all restarts.
    pub minimum: Minimum,
}

impl FireflyFullOptimizationResult {
    #[inline]
    pub fn new(
        iterations_performed_per_restart: Vec<usize>,
        minimum: Minimum,
    ) -> Self {
        Self {
            iterations_performed_per_restart,
            minimum,
        }
    }
}

pub struct FireflySingleRunOptimizationResult {
    pub iterations_performed: usize,
    pub minimum: PointValue,
}

impl FireflySingleRunOptimizationResult {
    pub fn new(iterations_performed: usize, minimum: PointValue) -> Self {
        Self {
            iterations_performed,
            minimum,
        }
    }
}
