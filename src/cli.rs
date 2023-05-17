use clap::{Parser, Subcommand};
use coco_rs::LogLevel;
use miette::{Context, IntoDiagnostic, miette, Result};

use crate::commands::firefly_optimization::cmd_run_firefly_optimization;
use crate::commands::simulated_annealing::run_cmd_simulated_annealing;

mod algorithms;
mod commands;
mod core;

#[derive(Parser, Eq, PartialEq)]
struct CLIArgs {
    #[command(subcommand)]
    pub command: CLICommands,
}

#[derive(Subcommand, Eq, PartialEq)]
enum CLICommands {
    // Example:
    // #[command(
    //     name = "run-foo",
    //     about = "Your description here."
    // )]
    // RunFoo,
    #[command(
    name = "firefly-optimization",
    about = "Runs the Firefly Optimization (variant of swarm optimization algorithm)."
    )]
    RunFireflyOptimization,

    #[command(
    name = "simulated-annealing",
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

    if args.command == CLICommands::RunFireflyOptimization {
        cmd_run_firefly_optimization()?;
    } else if args.command == CLICommands::RunSimulatedAnnealing {
        run_cmd_simulated_annealing()?;
    }
    else {
        panic!("Invalid command!");
    }

    Ok(())
}
