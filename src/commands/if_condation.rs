use std::{rc::Rc, sync::Mutex};

use miette::Result;

use crate::{
    commands::{Command, FunctionalCommand},
    execute::ExecutionStuck,
    var::{VarValue, Vars},
};

#[derive(knus::Decode, Debug, Clone)]
pub struct If {
    #[knus(property)]
    var: String,
    #[knus(property)]
    is: Option<String>, // TODO: make this an enum between str and bool
    #[knus(property, default = false)]
    not: bool,
    #[knus(children)]
    commands: Vec<Command>,
}

impl FunctionalCommand for If {
    fn exec(
        &self,
        vars: Vars,
        execution_stuck: Rc<Mutex<ExecutionStuck>>,
        command_span: knus::span::LineSpan,
    ) -> Result<()> {
        if let Some(var) = vars
            .lock()
            .unwrap()
            .iter()
            .find(|it| it.name == self.var.clone())
        {
            if let VarValue::Bool(_) = var.value
                && self.is.is_some()
            {
                return Err(miette::miette!("cant use `is` with a bool var"));
            } else if let VarValue::Str(_) = var.value
                && self.is.is_none()
            {
                return Err(miette::miette!("u should use `is` with a str var"));
            }

            let condition = match &var.value {
                VarValue::Bool(b) => {
                    if self.not {
                        !*b
                    } else {
                        *b
                    }
                }
                VarValue::Str(s) => s == self.is.as_ref().unwrap(),
            };

            if condition {
                execution_stuck.lock().unwrap().commands.extend(
                    self.commands
                        .clone()
                        .iter()
                        .map(|cmd| (cmd.clone(), command_span)),
                );
            }
        } else {
            return Err(miette::miette!(
                "cant compare var `{}` because it not found",
                self.var
            ));
        };

        Ok(())
    }
}
