// mod copy;
// mod env;
// mod input;
// mod log;
// mod rm;
// mod run;
// mod if;

use miette::Result;
use std::sync::{Arc, Mutex};

// use copy::Copy;
// use env::Env;
// use input::Input;
// use log::Log;
// use miette::Result;
// use rm::Rm;
// use run::Run;
// use if::If;

use crate::{execute::ExecutionStuck, var::Var};

#[derive(knus::Decode, Debug, Clone)]
pub enum Command {
    // Copy(Copy),
    // Log(Log),
    // Env(Env),
    // Run(Run),
    // Input(Input),
    // Rm(Rm),
    // If(If),
}

pub type Vars = Arc<Mutex<Vec<Var>>>;

pub trait FunctionalCommand {
    fn exec(&self, vars: Vars, execution_stuck: &mut ExecutionStuck) -> Result<()>;
}

impl FunctionalCommand for Command {
    fn exec(&self, vars: Vars, execution_stuck: &mut ExecutionStuck) -> Result<()> {
        // match self {
        //     Command::Env(it) => it.run(vars),
        //     Command::Copy(it) => it.run(vars),
        //     Command::Log(it) => it.run(vars),
        //     Command::Run(it) => it.run(vars),
        //     Command::Input(it) => it.run(vars),
        //     Command::Rm(it) => it.run(vars),
        //     Command::If(it) => it.run(vars),
        // }
        Ok(())
    }
}
