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
    #[command(
        name = "run-R-smoof-comparison",
        about = "Run the function for comparing R's smoof library and our embedded BBOB \
        library's functions. Run \"smoof_comparison.R\" in R to get the other half of this comparison."
    )]
    RunSmoofComparison,
}

fn main() -> Result<()> {
    coco_rs::set_log_level(LogLevel::Error);
    env_logger::try_init()
        .into_diagnostic()
        .wrap_err_with(|| miette!("Could not initialize logger."))?;

    let args = CLIArgs::parse();

    if args.command == CLICommands::RunSmoofComparison {
        cmd_smoof_comparison()?;
    } else {
        error!("Invalid command.");
    }

    Ok(())
}
