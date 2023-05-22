use clap::{Parser, Subcommand};
use coco_rs::LogLevel;
use miette::{miette, Context, IntoDiagnostic, Result};

use crate::commands::firefly_optimization::{
    cmd_run_firefly_optimization,
    CLIFireflyOptimizationArgs,
};
use crate::commands::simulated_annealing::run_cmd_simulated_annealing;

mod algorithms;
mod commands;
mod core;


#[derive(Parser)]
#[command(name = "optimization-cli", author, version)]
struct CLIArgs {
    #[command(subcommand)]
    pub command: CLICommands,
}

#[derive(Subcommand, Eq, PartialEq)]
enum CLICommands {
    #[command(
        name = "run-firefly-optimization",
        about = "Runs the Firefly Optimization (variant of swarm optimization algorithm)."
    )]
    RunFireflyOptimization(CLIFireflyOptimizationArgs),

    #[command(
        name = "run-simulated-annealing",
        about = "Runs simulated annealing (local search optimization technique)."
    )]
    RunSimulatedAnnealing,
}


fn main() -> Result<()> {
    coco_rs::set_log_level(LogLevel::Error);
    env_logger::try_init()
        .into_diagnostic()
        .wrap_err_with(|| miette!("Could not initialize logger."))?;

    let args = CLIArgs::parse();

    if let CLICommands::RunFireflyOptimization(args) = args.command {
        cmd_run_firefly_optimization(args)?;
    } else if args.command == CLICommands::RunSimulatedAnnealing {
        run_cmd_simulated_annealing()?;
    }

    Ok(())
}
