use std::{env, rc::Rc, sync::Mutex};

use crate::{
    commands::FunctionalCommand,
    execute::ExecutionStuck,
    var::{Var, VarValue, Vars},
};

const ENV_VARS_TO_IGNORE: [&str; 4] = ["SHELL", "PWD", "TERM", "PATH"];

#[derive(Debug, Clone)]
pub enum EnvActionType {
    /// load an env var from the env and write it in a var
    Load,
    /// inject an var value into env
    Inject,
    /// drop an var from env - if var property is empty, drop all env vars
    Drop,
}

impl std::str::FromStr for EnvActionType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "load" => Ok(EnvActionType::Load),
            "inject" => Ok(EnvActionType::Inject),
            "drop" => Ok(EnvActionType::Drop),
            _ => Err("env type name must be `load`, `inject` or `drop`")?,
        }
    }
}

#[derive(knus::Decode, Debug, Clone)]
pub struct Env {
    #[knus(property)]
    var: String,
    #[knus(property)]
    into: Option<String>,
    #[knus(property, default = false)]
    if_exists: bool,
    #[knus(type_name)]
    action: EnvActionType,
}

impl FunctionalCommand for Env {
    fn exec(
        &self,
        vars: Vars,
        _execution_stuck: Rc<Mutex<ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        match self.action {
            EnvActionType::Inject => {
                if self.into.is_some() {
                    return Err(miette::miette!("cant use into with inject"));
                }

                let var_value = vars
                    .lock()
                    .unwrap()
                    .iter()
                    .find(|it| it.name == self.var.clone())
                    .ok_or_else(|| miette::miette!("env var not found: {}", &self.var))?
                    .value
                    .clone();

                unsafe {
                    env::set_var(&self.var, var_value.to_string());
                }
            }
            EnvActionType::Load => {
                if env::var(&self.var).is_err() && !self.if_exists {
                    return Err(miette::miette!(format!(
                        "cant load var `{}` because it not found",
                        &self.var
                    )));
                } else if env::var(&self.var).is_err() && self.if_exists {
                    return Ok(());
                }

                let var_value = env::var(&self.var).unwrap();

                if let Some(var) = vars
                    .lock()
                    .unwrap()
                    .iter_mut()
                    .find(|it| it.name == self.var)
                {
                    var.value = VarValue::Str(var_value);
                } else {
                    vars.lock().unwrap().push(Var {
                        name: self.into.clone().unwrap_or(self.var.clone()),
                        value: VarValue::Str(var_value),
                    });
                }
            }
            EnvActionType::Drop => {
                if self.into.is_some() {
                    return Err(miette::miette!("cant use into with drop"));
                }

                if self.var == "*" {
                    for (var_name, _) in env::vars() {
                        if ENV_VARS_TO_IGNORE.contains(&var_name.as_str()) {
                            continue;
                        };

                        unsafe { env::remove_var(var_name) };
                    }
                    return Ok(());
                }

                if env::var(&self.var).is_err() && !self.if_exists {
                    return Err(miette::miette!(format!(
                        "cant drop var `{}` because it not found",
                        &self.var
                    )));
                }

                unsafe { env::remove_var(&self.var) };
            }
        }

        Ok(())
    }
}
