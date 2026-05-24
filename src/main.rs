mod cli;
mod commands;
mod config;
mod flow_control;
mod parser;
mod thread;
mod utils;
mod var;

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
