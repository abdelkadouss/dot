mod cli;
mod commands;
mod config;
mod utils;

use cli::Cli;
use config::Config;
use miette::{Diagnostic, IntoDiagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum NewError {}

fn main() -> Result<(), miette::Report> {
    let config = Config::new()?;

    Cli::route(config)?;

    Ok(())
}
