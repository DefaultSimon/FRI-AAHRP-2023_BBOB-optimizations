mod commands;
mod core;
mod algorithms;

use clap::{Parser, Subcommand};
use coco_rs::LogLevel;
use miette::{miette, Context, IntoDiagnostic, Result};
use crate::commands::firefly_optimization::cmd_run_firefly_optimization;

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
}

fn main() -> Result<()> {
    coco_rs::set_log_level(LogLevel::Error);
    env_logger::try_init()
        .into_diagnostic()
        .wrap_err_with(|| miette!("Could not initialize logger."))?;

    let args = CLIArgs::parse();
    
    if args.command == CLICommands::RunFireflyOptimization {
        cmd_run_firefly_optimization()?;
    } else {
        panic!("Invalid command!");
    }

    Ok(())
}
