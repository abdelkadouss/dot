use std::{rc::Rc, sync::Mutex};

use crate::execute::ExecutionStuck;

#[derive(knus::Decode, Debug, Clone)]
pub struct Var {
    #[knus(property)]
    name: String,
    #[knus(property)]
    value: String,
    #[knus(type_name, default = VarType::Str)]
    var_type: VarType,
}

#[derive(Debug, Clone)]
pub enum VarType {
    Str,
    Bool,
}

impl std::str::FromStr for VarType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "str" => Ok(VarType::Str),
            "bool" => Ok(VarType::Bool),
            _ => Err("var type name must be `str` or `bool`")?,
        }
    }
}

impl crate::commands::FunctionalCommand for Var {
    fn exec(
        &self,
        vars: crate::var::Vars,
        _execution_stuck: Rc<Mutex<ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        let var = match self.var_type {
            VarType::Str => crate::var::Var {
                name: self.name.clone(),
                value: crate::var::VarValue::Str(self.value.clone()),
            },
            VarType::Bool => {
                if self.value != "true" && self.value != "false" {
                    return Err(miette::miette!("bool var value must be `true` or `false`"));
                }

                crate::var::Var {
                    name: self.name.clone(),
                    value: crate::var::VarValue::Bool(self.value == "true"),
                }
            }
        };

        vars.lock().unwrap().push(var);

        Ok(())
    }
}
