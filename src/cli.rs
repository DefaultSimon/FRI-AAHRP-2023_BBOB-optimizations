mod commands;
mod core;
mod algorithms;

use clap::{Parser, Subcommand};
use coco_rs::LogLevel;
use miette::{miette, Context, IntoDiagnostic, Result};

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
}

fn main() -> Result<()> {
    coco_rs::set_log_level(LogLevel::Error);
    env_logger::try_init()
        .into_diagnostic()
        .wrap_err_with(|| miette!("Could not initialize logger."))?;

    // let args = CLIArgs::parse();

    todo!(
        "Add two commands, each running one of the two optimization algorithms (see CLICommands)."
    );

    // Ok(())
}
