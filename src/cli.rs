use clap::{ColorChoice, Parser, Subcommand};
use miette::Result;

use crate::{config::Config, execute::Execute};

pub const DEFAULT_SOURCE_DECLARATION_FILE_NAME: &str = "Dotfile.kdl";
pub const DEFAULT_SCRIPTS_DIR: &str = "scripts";

#[derive(Parser)]
#[command(name = "dot")]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(color = ColorChoice::Always)] // Always show colors
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand)]
pub(crate) enum CliCommands {
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
            CliCommands::Build => config.source_dir.join(DEFAULT_SOURCE_DECLARATION_FILE_NAME),
            CliCommands::Script { script_name } => config
                .source_dir
                .join(DEFAULT_SCRIPTS_DIR)
                .join(script_name)
                .with_extension("kdl"),
        };

        Execute::execute_script(to_run_script)
    }
}
