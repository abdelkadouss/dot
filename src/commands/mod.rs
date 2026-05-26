mod call;
mod copy;
mod env;
mod fork;
mod if_condation;
mod input;
mod link;
mod log;
mod rm;
mod run;
mod var;

use std::{rc::Rc, sync::Mutex};

use miette::Result;

use call::Call;
use copy::Copy;
use env::Env;
use fork::Fork;
use if_condation::If;
use input::Input;
use link::Link;
use log::Log;
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
    Link(Link),
    Input(Input),
    Rm(Rm),
    If(If),
    Fork(Fork),
    Call(Call),
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
            Command::Input(it) => it.exec(vars, execution_stuck, command_span),
            Command::Rm(it) => it.exec(vars, execution_stuck, command_span),
            Command::Link(it) => it.exec(vars, execution_stuck, command_span),
            Command::If(it) => it.exec(vars, execution_stuck, command_span),
            Command::Fork(it) => it.exec(vars, execution_stuck, command_span),
            Command::Call(it) => it.exec(vars, execution_stuck, command_span),
        }
    }
}
