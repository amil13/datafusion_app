/*
* This file serves as the main application entry point
*
* Author: Amil Shrivastava*
*/ 

// Imports for modules and logging
mod cli;
mod errors;
mod processing;
mod utils;
mod tests;

use cli::MDataAppArgs;
use errors::MDataAppError;
use processing::mdata_app;
use log::{debug, LevelFilter};
use env_logger::{Builder, Env};
use clap::Parser;

// Entry point for the application
#[tokio::main]
async fn main() -> Result<(), MDataAppError> {
    // Parse command-line arguments
    let cli = MDataAppArgs::parse();
    // Set up logging based on the verbosity level specified
    let log_level = match cli.verbose {
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    // Configure the logging environment
    let env = Env::new().filter("MLOG");
    Builder::new()
        .filter(Some("mdata_app"), log_level)
        .parse_env(env)
        .init();

    // Display parsed arguments for debugging
    debug!("Arguments {:#?}", cli);

    // Run the main application logic with parsed arguments
    mdata_app(cli).await
}
