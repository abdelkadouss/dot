use std::{cmp::Ordering, fs::read_to_string, sync::Arc, thread};

use clap::{ColorChoice, Parser, Subcommand};
use miette::{IntoDiagnostic, Result};

use crate::{
    commands::{Command, FunctionalCommand, Vars},
    config::Config,
    var::Var,
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

pub type ThreadCounter = usize;

pub trait ThreadedAndFlowControledCommandBlock {
    fn get_thread_counter(&self) -> ThreadCounter;
    fn get_commands_block(&self) -> Vec<&Command>;
}

#[derive(Debug)]
pub enum ControlFlowBlock {}

impl ThreadedAndFlowControledCommandBlock for ControlFlowBlock {
    fn get_thread_counter(&self) -> ThreadCounter {
        todo!()
    }
    fn get_commands_block(&self) -> Vec<&Command> {
        todo!()
    }
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
    // make the vars shared state
    let vars = Vars::new(Vec::<Var>::new().into());

    // make the threads handler
    let mut threads_stuck = Vec::<thread::JoinHandle<Result<()>>>::new();

    for ordered_commands_block in parse_and_order_commands::<ControlFlowBlock>(commands) {
        let block_thread_count = ordered_commands_block.get_thread_counter();

        let vars_clone = Arc::clone(&vars);
        let execute_cluster = move || -> Result<()> {
            for command in ordered_commands_block.get_commands_block() {
                command.run(Arc::clone(&vars_clone))?
            }

            Ok(())
        };

        match block_thread_count.cmp(&threads_stuck.len()) {
            Ordering::Equal => {
                execute_cluster();
            }
            Ordering::Greater => {
                let new_thread = thread::spawn(move || execute_cluster());

                threads_stuck.push(new_thread); // here is the problem
            }
            Ordering::Less => {
                todo!()
            }
        }
    }

    Ok(())
}

fn parse_and_order_commands<T: ThreadedAndFlowControledCommandBlock>(
    commands: Vec<Command>,
) -> Vec<T> {
    todo!()
}
