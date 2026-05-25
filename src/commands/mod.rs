mod copy;
// mod env;
// mod input;
mod log;
// mod rm;
// mod run;
// mod if;
mod fork;

use std::{rc::Rc, sync::Mutex};

use miette::Result;

use copy::Copy;
// use env::Env;
// use input::Input;
use log::Log;
// use miette::Result;
// use rm::Rm;
// use run::Run;
// use if::If;
use fork::Fork;

use crate::{execute::ExecutionStuck, var::Vars};

#[derive(knus::Decode, Debug, Clone)]
pub enum Command {
    Copy(Copy),
    Log(Log),
    // Env(Env),
    // Run(Run),
    // Input(Input),
    // Rm(Rm),
    // If(If),
    Fork(Fork),
}

pub trait FunctionalCommand {
    fn exec(&self, vars: Vars, execution_stuck: Rc<Mutex<ExecutionStuck>>) -> Result<()>;
}

impl FunctionalCommand for Command {
    fn exec(&self, vars: Vars, execution_stuck: Rc<Mutex<ExecutionStuck>>) -> Result<()> {
        match self {
            //     Command::Env(it) => it.run(vars),
            Command::Copy(it) => it.exec(vars, execution_stuck),
            Command::Log(it) => it.exec(vars, execution_stuck),
            //     Command::Run(it) => it.run(vars),
            //     Command::Input(it) => it.run(vars),
            //     Command::Rm(it) => it.run(vars),
            //     Command::If(it) => it.run(vars),
            Command::Fork(it) => it.exec(vars, execution_stuck),
        }
    }
}
