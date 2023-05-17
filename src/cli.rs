mod algorithms;
mod commands;
mod core;

use clap::{Parser, Subcommand};
use coco_rs::LogLevel;
use miette::{miette, Context, IntoDiagnostic, Result};

use crate::commands::firefly_optimization::{
    cmd_run_firefly_optimization,
    CLIFireflyOptimizationArgs,
};

#[derive(Parser)]
struct CLIArgs {
    #[command(subcommand)]
    pub command: CLICommands,
}

#[derive(Subcommand)]
enum CLICommands {
    #[command(
        name = "run-firefly-optimization",
        about = "Runs the Firefly Optimization (variant of swarm optimization algorithm)."
    )]
    RunFireflyOptimization(CLIFireflyOptimizationArgs),
}

fn main() -> Result<()> {
    coco_rs::set_log_level(LogLevel::Error);
    env_logger::try_init()
        .into_diagnostic()
        .wrap_err_with(|| miette!("Could not initialize logger."))?;

    let args = CLIArgs::parse();

    // FIXME: Remove this `allow` when multiple commands are added.
    #[allow(irrefutable_let_patterns)]
    if let CLICommands::RunFireflyOptimization(args) = args.command {
        cmd_run_firefly_optimization(args)?;
    }

    Ok(())
}
