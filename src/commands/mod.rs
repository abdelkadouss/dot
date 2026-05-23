mod copy;
mod env;
mod input;
mod log;
mod rm;
mod run;

use std::path::Path;

use copy::Copy;
use env::Env;
use input::Input;
use log::Log;
use miette::Result;
use rm::Rm;
use run::Run;

#[derive(Debug)]
pub struct Var {
    pub name: String,
    pub value: VarType,
}

#[derive(knus::Decode, Debug)]
pub enum Command {
    Copy(Copy),
    Log(Log),
    Env(Env),
    Run(Run),
    Input(Input),
    Rm(Rm),
}

pub trait FunctionalCommand {
    fn run(&self) -> Result<()>;
}

impl FunctionalCommand for Command {
    fn run(&self) -> Result<()> {
        match self {
            Command::Env(it) => it.run(),
            Command::Copy(it) => it.run(),
            Command::Log(it) => it.run(),
            Command::Run(it) => it.run(),
            Command::Input(it) => it.run(),
            Command::Rm(it) => it.run(),
        }
    }
}
