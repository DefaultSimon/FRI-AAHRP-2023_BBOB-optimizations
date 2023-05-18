use std::time::Duration;

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use miette::{IntoDiagnostic, Result};

use crate::algorithms::firefly::swarm::FireflySwarm;
use crate::algorithms::firefly::{FireflyRunOptions, OptimizationRunType};

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
        run_type: OptimizationRunType,
        options: &FireflyRunOptions,
    ) -> Result<FireflySingleRunProgressBar> {
        FireflySingleRunProgressBar::from_multi_progress_bar(
            &self.multi_bar,
            run_type,
            options,
        )
    }
}


pub struct FireflySingleRunProgressBar {
    progress_bar: ProgressBar,
    run_type: OptimizationRunType,
}

impl FireflySingleRunProgressBar {
    pub fn from_multi_progress_bar(
        multi_progress_bar: &MultiProgress,
        run_type: OptimizationRunType,
        options: &FireflyRunOptions,
    ) -> Result<Self> {
        let running_style = match run_type {
            OptimizationRunType::Exploration { run_number, total_runs } => {
                ProgressStyle::with_template(&format!(
                    "[ {} | run {}/{}] |{{bar:40}}| iteration {{pos}}/{{len}}: {{msg}}",
                    "  explore  ".bright_yellow().bold(), run_number, total_runs,
                ))
                    .into_diagnostic()?
            }
            OptimizationRunType::Refinement { run_number, total_runs, .. } => {
                ProgressStyle::with_template(&format!(
                    "[ {} | run {}/{}] |{{bar:40}}| iteration {{pos}}/{{len}}: {{msg}}",
                    "refine best".bright_yellow().bold(), run_number, total_runs,
                ))
                    .into_diagnostic()?
            }
        };

        let progress_bar = multi_progress_bar.add(
            ProgressBar::new(options.maximum_iterations as u64)
                .with_style(running_style),
        );

        Ok(Self {
            progress_bar,
            run_type,
        })
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
        iterations_performed: usize,
        minimum_value: f64,
        global_minimum: f64,
        options: &FireflyRunOptions,
    ) -> Result<()> {
        let finished_style =
            ProgressStyle::with_template("{msg}").into_diagnostic()?;
        self.progress_bar.set_style(finished_style);

        let final_message = match self.run_type {
            OptimizationRunType::Exploration {
                run_number,
                total_runs,
            } => {
                format!(
                    "[ {} | run {}/{}]  {}/{:04} iterations | minimum: {:.5}, distance: {:.5}",
                    "  explore  ".bright_yellow().bold(),
                    run_number,
                    total_runs,
                    iterations_performed,
                    options.maximum_iterations,
                    minimum_value,
                    minimum_value - global_minimum
                )
            }
            OptimizationRunType::Refinement {
                run_number,
                total_runs,
                best_value_before_refinement,
            } => {
                let updated_minimum_str =
                    if minimum_value < best_value_before_refinement {
                        format!("-> {:.5}", minimum_value).bright_green()
                    } else {
                        format!("-> {:.5}", minimum_value).yellow()
                    };

                format!(
                    "[ {} | run {}/{}]  {}/{:04} iterations | minimum: {:.5} {}, distance: {:.5}",
                    "refine best".bright_cyan().bold(),
                    run_number,
                    total_runs,
                    iterations_performed,
                    options.maximum_iterations,
                    best_value_before_refinement,
                    updated_minimum_str,
                    minimum_value - global_minimum
                )
            }
        };

        self.progress_bar.finish_with_message(final_message);

        self.progress_bar.disable_steady_tick();
        self.progress_bar.tick();

        Ok(())
    }
}
