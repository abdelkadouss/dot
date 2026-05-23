#![allow(dead_code)]
use std::{fs, path::Path};

use miette::IntoDiagnostic;

use crate::commands::{FunctionalCommand, Var};

const DEFAULT_PLACE_HOSLDER: &str = "{{:?:}}";

#[derive(Debug)]
pub enum IfActionType {
    Include,
    Exclude,
}

impl std::str::FromStr for IfActionType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "include" => Ok(IfActionType::Include),
            "exclude" => Ok(IfActionType::Exclude),
            _ => Err("if type name must be `include` or `exclude`")?,
        }
    }
}

#[derive(knus::Decode, Debug)]
pub struct If {
    #[knus(property)]
    var: String,
    #[knus(property)]
    is: Option<String>,
    #[knus(arguments)]
    files: Vec<String>,
    #[knus(type_name)]
    action_type: IfActionType,
}

#[derive(knus::Decode, Debug)]
pub struct Ignore {
    #[knus(arguments)]
    files: Vec<String>,
}

#[derive(knus::Decode, Debug)]
pub struct Format {
    #[knus(arguments)]
    files: Vec<String>,
}

#[derive(knus::Decode, Debug)]
pub struct Copy {
    #[knus(argument)]
    from: String,
    #[knus(argument)]
    to: String,
    #[knus(children(name = "format"))]
    formats: Vec<Format>,
    #[knus(children(name = "if"))]
    conditions: Vec<If>,
    #[knus(children(name = "ignore"))]
    ignore: Vec<Ignore>,
}

impl FunctionalCommand for Copy {
    fn run(&self) -> miette::Result<()> {
        for format in &self.formats {
            format.run(source, target, vars)?
        }

        for condition in &self.conditions {
            condition.run(source, target, vars)?
        }

        fs::copy(source.join(&self.from), target.join(&self.to))
            .map(|_| ())
            .into_diagnostic()
    }
}

impl FunctionalCommand for Format {
    fn run(&self) -> miette::Result<()> {
        let ph_vec = DEFAULT_PLACE_HOSLDER.split('?').collect::<Vec<&str>>();
        let (ph_left_part, ph_right_part) = (ph_vec[0], ph_vec[1]);

        let vars_with_place_holder = vars
            .iter()
            .map(|var| {
                let value = match &var.value {
                    VarType::Str(str) => str.to_string(),
                    VarType::Bool(bool) => bool.to_string(),
                };

                (format!("{ph_left_part}{}{ph_right_part}", var.name), value)
            })
            .collect::<Vec<(String, String)>>();

        for file in &self.files {
            for (ph, val) in &vars_with_place_holder {
                let formated_file = fs::read_to_string(source.join(file))
                    .into_diagnostic()?
                    .replace(ph, val);

                fs::write(target.join(file), formated_file).into_diagnostic()?;
            }
        }

        Ok(())
    }
}

impl FunctionalCommand for If {
    fn run(&self, source: &Path, target: &Path, vars: &mut Vec<Var>) -> miette::Result<()> {
        let var = vars.iter().find(|var| *var.name == self.var);

        if var.is_none() {
            todo!("return error")
        }

        let var = var.unwrap();

        let var_value = match &var.value {
            VarType::Str(str) => str.to_string(),
            VarType::Bool(bool) => bool.to_string(),
        };

        if let None = &self.is
            && let VarType::Str(_) = var.value
        {
            todo!("return error")
        }

        if var_value != self.is.clone().unwrap_or("true".to_string()) {
            return Ok(());
        }

        match &self.action_type {
            IfActionType::Include => {
                for file in &self.files {
                    fs::copy(source.join(file), target.join(file)).into_diagnostic()?;
                }
            }
            IfActionType::Exclude => {}
        };

        todo!()
    }
}
