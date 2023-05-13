mod commands;
mod core;

use clap::{Parser, Subcommand};
use coco_rs::LogLevel;
use log::error;
use miette::{miette, Context, IntoDiagnostic, Result};

use crate::commands::cmd_smoof_comparison;

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
