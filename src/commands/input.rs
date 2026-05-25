use std::{fmt::Display, rc::Rc, sync::Mutex};

use crate::{commands::FunctionalCommand, execute::ExecutionStuck, utils, var::Vars};

use miette::IntoDiagnostic;
use owo_colors::OwoColorize;

#[derive(Debug, Clone)]
pub enum InputType {
    Str,
    Bool,
}

impl std::str::FromStr for InputType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "str" => Ok(InputType::Str),
            "bool" => Ok(InputType::Bool),
            _ => Err("input type name must be `str` or `bool`")?,
        }
    }
}

#[derive(knus::Decode, Debug, Clone)]
pub struct Input {
    /// to store data into
    #[knus(property)]
    var: String,
    /// prompt to show to the user
    #[knus(property)]
    prompt: Option<String>,
    /// default value when the user press enter
    #[knus(property)]
    default: Option<String>,
    /// input type
    #[knus(type_name)]
    input_type: InputType,
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Str => write!(f, "str"),
            InputType::Bool => write!(f, "bool"),
        }
    }
}

impl FunctionalCommand for Input {
    fn exec(
        &self,
        vars: Vars,
        _execution_stuck: Rc<Mutex<ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        if let Some(prompt) = self.prompt.clone().as_mut() {
            // format the prompt using the vars
            utils::var::format_string_using_vars(prompt, vars.lock().unwrap());

            if let Some(default) = self.default.clone().as_mut() {
                // also format the default value
                utils::var::format_string_using_vars(default, vars.lock().unwrap());

                println!(
                    "$ {} {}",
                    prompt.blue().bold(),
                    format!("<{}> [{} - enter]", self.input_type, default).black()
                );
            } else {
                println!("{}", prompt.blue().bold());
            }
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).into_diagnostic()?;

        let mut input = input.trim_end().to_string();

        if input.is_empty()
            && let Some(default) = &self.default
        {
            input = default.clone();
        }

        if let InputType::Bool = self.input_type
            && input != "true"
            && input != "false"
        {
            Err(miette::miette!(
                "invalid bool value, must be `true` or `false`"
            ))?;
        }

        vars.lock().unwrap().push(crate::var::Var {
            name: self.var.clone(),
            value: match self.input_type {
                InputType::Str => crate::var::VarValue::Str(input),
                InputType::Bool => crate::var::VarValue::Bool(input == "true"),
            },
        });

        Ok(())
    }
}
