use std::fs::read_to_string;

use clap::{ColorChoice, Parser, Subcommand};
use miette::{IntoDiagnostic, Result};

use crate::{
    commands::{Command, FunctionalCommand},
    config::Config,
};

const DEFAULT_SOURCE_DECLARATION_FILE_NAME: &str = "Dotfile.kdl";
const DEFAULT_SCRIPTS_DIR: &str = "scripts";
const TMPS_BASE_DIR: &str = "/tmp/dot";

#[derive(Parser)]
#[command(name = "dot")]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(color = ColorChoice::Always)] // Always show colors
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// read the dotfile.kdl file and execute it
    Build,

    /// run the script with the given name from the scripts dir
    Script {
        /// the name of the script to run
        script_name: String,
    },
}

impl Cli {
    pub fn route(config: Config) -> Result<()> {
        let cli = Cli::parse();

        let to_run_script = match cli.command {
            Commands::Build => config.source_dir.join(DEFAULT_SOURCE_DECLARATION_FILE_NAME),
            Commands::Script { script_name } => config
                .source_dir
                .join(DEFAULT_SCRIPTS_DIR)
                .join(script_name),
        };

        let parsed_script = knus::parse::<Vec<Command>>(
            to_run_script.clone().to_str().unwrap(),
            read_to_string(to_run_script).into_diagnostic()?.as_str(),
        )?;

        run_kdl_commands(parsed_script)
    }
}

fn run_kdl_commands(commands: Vec<Command>) -> Result<()> {
    // parse the script (via kdl lib)
    // loop over the commands
    // execute the commands
    for command in commands {
        command.run()?
    }

    Ok(())
}
