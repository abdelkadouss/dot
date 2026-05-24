use std::{sync::Arc, thread};

use clap::{ColorChoice, Parser, Subcommand};
use miette::{Diagnostic, Result, SourceSpan};
use thiserror::Error;

use crate::{
    commands::{FunctionalCommand, Vars},
    config::Config,
    parser::{OrgnizedBlock, Parser as DotParser, ThreadIndicator},
    var::Var,
};

const DEFAULT_SOURCE_DECLARATION_FILE_NAME: &str = "Dotfile.kdl";
const DEFAULT_SCRIPTS_DIR: &str = "scripts";

#[derive(Debug, Error, Diagnostic)]
pub enum ExecutionError {
    #[error("u already in the main thread - can't join")]
    #[diagnostic(code(commands::run_time_error))]
    AlreadyInMainThread {
        #[source_code]
        source_code: String,
        #[label("This bit here")]
        source_span: SourceSpan,
    },
}

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
                .join(script_name),
        };

        let parsed_script = DotParser::parse_and_order_commands(&to_run_script)?;

        run_kdl_commands(parsed_script)
    }
}

fn run_kdl_commands(commands: Vec<OrgnizedBlock>) -> Result<()> {
    // make the vars shared state
    let vars = Vars::new(Vec::<Var>::new().into());

    // make the threads handler
    let mut threads_stuck = Vec::<thread::JoinHandle<Result<()>>>::new();

    for ordered_commands_block in commands {
        // get the block meta data
        let block_thread_count = ordered_commands_block.thread_indicator;
        let block_source_code = ordered_commands_block.source_code;
        let block_gate_span = ordered_commands_block.gate_span;

        let vars_clone = Arc::clone(&vars);
        // TODO: u have to handle the return values of the threads insha'Allah
        let execute_cluster = move || -> Result<()> {
            for command in ordered_commands_block.commands {
                command.exec(Arc::clone(&vars_clone))?
            }

            Ok(())
        };

        match block_thread_count {
            ThreadIndicator::Main => {
                execute_cluster()?;
            }
            ThreadIndicator::Push => {
                threads_stuck.push(thread::spawn(execute_cluster));
            }
            ThreadIndicator::Join => {
                if let Some(handle) = threads_stuck.pop() {
                    handle.join().unwrap()?; // own it, join it, propagate the Result
                } else {
                    Err(ExecutionError::AlreadyInMainThread {
                        source_code: block_source_code,
                        source_span: block_gate_span,
                    })?;
                }

                execute_cluster()?;
            }
        }
    }

    Ok(())
}
