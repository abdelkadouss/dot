mod copy;
mod env;
// mod input;
mod fork;
mod if_condation;
mod log;
mod rm;
mod run;
mod var;

use std::{rc::Rc, sync::Mutex};

use miette::Result;

use copy::Copy;
use env::Env;
// use input::Input;
use log::Log;
// use miette::Result;
use fork::Fork;
use if_condation::If;
use rm::Rm;
use run::Run;

use crate::{execute::ExecutionStuck, var::Vars};

#[derive(knus::Decode, Debug, Clone)]
pub enum Command {
    Copy(Copy),
    Log(Log),
    Env(Env),
    Run(Run),
    Var(var::Var),
    // Input(Input),
    Rm(Rm),
    If(If),
    Fork(Fork),
}

pub trait FunctionalCommand {
    fn exec(
        &self,
        vars: Vars,
        execution_stuck: Rc<Mutex<ExecutionStuck>>,
        command_span: knus::span::LineSpan,
    ) -> Result<()>;
}

impl FunctionalCommand for Command {
    fn exec(
        &self,
        vars: Vars,
        execution_stuck: Rc<Mutex<ExecutionStuck>>,
        command_span: knus::span::LineSpan,
    ) -> Result<()> {
        match self {
            Command::Env(it) => it.exec(vars, execution_stuck, command_span),
            Command::Copy(it) => it.exec(vars, execution_stuck, command_span),
            Command::Log(it) => it.exec(vars, execution_stuck, command_span),
            Command::Run(it) => it.exec(vars, execution_stuck, command_span),
            Command::Var(it) => it.exec(vars, execution_stuck, command_span),
            //     Command::Input(it) => it.run(vars),
            Command::Rm(it) => it.exec(vars, execution_stuck, command_span),
            Command::If(it) => it.exec(vars, execution_stuck, command_span),
            Command::Fork(it) => it.exec(vars, execution_stuck, command_span),
        }
    }
}
