// mod copy;
// mod env;
// mod input;
// mod log;
// mod rm;
// mod run;

use miette::Result;
use std::sync::{Arc, Mutex};

// use copy::Copy;
// use env::Env;
// use input::Input;
// use log::Log;
// use miette::Result;
// use rm::Rm;
// use run::Run;

use crate::var::Var;

#[derive(knus::Decode, Debug)]
pub enum Command {
    // Copy(Copy),
    // Log(Log),
    // Env(Env),
    // Run(Run),
    // Input(Input),
    // Rm(Rm),
}

pub type Vars = Arc<Mutex<Vec<Var>>>;

pub trait FunctionalCommand {
    fn run(&self, vars: Vars) -> Result<()>;
}

impl FunctionalCommand for Command {
    fn run(&self, vars: Vars) -> Result<()> {
        // match self {
        //     Command::Env(it) => it.run(vars),
        //     Command::Copy(it) => it.run(vars),
        //     Command::Log(it) => it.run(vars),
        //     Command::Run(it) => it.run(vars),
        //     Command::Input(it) => it.run(vars),
        //     Command::Rm(it) => it.run(vars),
        // }
        Ok(())
    }
}
