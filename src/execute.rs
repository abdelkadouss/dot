use std::path::PathBuf;

use miette::Result;

use crate::{
    commands::Vars,
    commands::{Command, FunctionalCommand},
    parser::Parser,
    var::Var,
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

        let mut execution_stuck = parser.parse()?;

        for command in execution_stuck.commands.clone() {
            command.exec(vars.clone(), &mut execution_stuck)?;
        }

        Ok(())
    }
}
