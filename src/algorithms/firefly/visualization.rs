use std::time::Duration;

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use miette::{IntoDiagnostic, Result};

use crate::algorithms::firefly::swarm::FireflySwarm;
use crate::algorithms::firefly::FireflyRunOptions;

pub struct FireflyOptimizationMultiProgressBar {
    multi_bar: MultiProgress,
}

impl FireflyOptimizationMultiProgressBar {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            multi_bar: MultiProgress::new(),
        }
    }

    pub fn new_run(
        &self,
        run_number: usize,
        total_runs: usize,
        maximum_iterations: usize,
    ) -> Result<FireflySingleRunProgressBar> {
        FireflySingleRunProgressBar::from_multi_progress_bar(
            &self.multi_bar,
            run_number,
            total_runs,
            maximum_iterations as u64,
        )
    }
}


pub struct FireflySingleRunProgressBar {
    progress_bar: ProgressBar,
}

impl FireflySingleRunProgressBar {
    pub fn from_multi_progress_bar(
        multi_progress: &MultiProgress,
        run_number: usize,
        total_runs: usize,
        maximum_iterations: u64,
    ) -> Result<Self> {
        let running_style = ProgressStyle::with_template(&format!(
            "[run {}/{}] | {{bar:40}} | iteration {{pos}}/{{len}}: {{msg}}",
            run_number, total_runs,
        ))
        .into_diagnostic()?;

        let progress_bar = multi_progress
            .add(ProgressBar::new(maximum_iterations).with_style(running_style));

        Ok(Self { progress_bar })
    }

    pub fn start(&self) {
        self.progress_bar
            .enable_steady_tick(Duration::from_secs_f64(1f64 / 5f64));
    }

    pub fn update(
        &self,
        iterations_performed: usize,
        options: &FireflyRunOptions,
        swarm: &FireflySwarm,
    ) {
        self.progress_bar.set_position(iterations_performed as u64);

        let jitter_str = format!(
            "jitter={:.4}",
            swarm.current_movement_jitter_coefficient,
        );
        let iterations_since_improvement_str = format!(
            "iterations_since_improvement={:04}/{}",
            swarm.iterations_since_improvement,
            options.consider_stuck_after_n_iterations,
        );

        self.progress_bar.set_message(format!(
            "{} {} value={:.6}",
            // Colour the jitter value red when heating up and green when cooling down.
            if swarm.iterations_since_improvement
                > options.movement_jitter_min_stuck_runs_to_reheat
            {
                jitter_str.bright_red()
            } else {
                jitter_str.bright_green()
            },
            // Colour the stuck iteration count red when at 80%+ of run abort condition.
            if swarm.iterations_since_improvement as f32
                > (options.consider_stuck_after_n_iterations as f32 * 0.8)
            {
                iterations_since_improvement_str.red()
            } else {
                iterations_since_improvement_str.white()
            },
            swarm
                .current_best_solution
                .as_ref()
                .expect("BUG: Invalid swarm, no solution at all.")
                .value,
        ));
    }

    pub fn finish(
        &self,
        run_number: usize,
        total_runs: usize,
        iterations_performed: usize,
        minimum_value: f64,
        global_minimum: f64,
        options: &FireflyRunOptions,
    ) -> Result<()> {
        let finished_style =
            ProgressStyle::with_template("{msg}").into_diagnostic()?;
        self.progress_bar.set_style(finished_style);

        self.progress_bar.finish_with_message(format!(
            "[run {}/{}]  {}/{:04} iterations | minimum: {:.5}, distance: {:.5}",
            run_number,
            total_runs,
            iterations_performed,
            options.maximum_iterations,
            minimum_value,
            minimum_value - global_minimum
        ));

        self.progress_bar.disable_steady_tick();
        self.progress_bar.tick();

        Ok(())
    }
}
