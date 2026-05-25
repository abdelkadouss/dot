use std::{path::PathBuf, rc::Rc, sync::Mutex};

use miette::Result;

use crate::{
    commands::{Command, FunctionalCommand},
    parser::Parser,
    var::{Var, Vars},
};

#[derive(Debug, Clone)]
pub struct ExecutionStuck {
    pub commands: Vec<Command>,
}

pub struct Execute {}

impl Execute {
    pub fn execute_script(path_to_script: PathBuf) -> Result<()> {
        // make the vars shared state
        let vars = Vars::new(Vec::<Var>::new().into());

        let parser = Parser::new(&path_to_script)?;

        let execution_stuck = Rc::new(Mutex::new(parser.parse()?));

        loop {
            let command = execution_stuck.lock().unwrap().commands.pop();

            if let Some(cmd) = command {
                cmd.exec(vars.clone(), Rc::clone(&execution_stuck))?
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl ExecutionStuck {
    /// make a new execution stuck
    pub fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }

    /// return an empty execution stuck
    pub fn empty() -> Self {
        Self {
            commands: Vec::<Command>::new(),
        }
    }
}
