mod cli;
mod commands;
mod config;
mod execute;
mod parser;
mod utils;
mod var;

use cli::Cli;
use config::Config;
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum NewError {}

fn main() -> Result<(), miette::Report> {
    let config = Config::new()?;

    Cli::route(config)?;

    Ok(())
}
