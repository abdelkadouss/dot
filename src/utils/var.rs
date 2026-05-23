use crate::commands::{Var, VarType};

pub fn format_string_using_vars(str: &mut String, vars: &Vec<Var>) {
    vars.iter().for_each(|var| {
        let value = match &var.value {
            VarType::Str(str) => str.to_string(),
            VarType::Bool(bool) => bool.to_string(),
        };

        *str = str.replace(var.name.as_str(), value.as_str())
    });
}
