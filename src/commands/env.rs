use std::{env, fs, path::PathBuf};

use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

use crate::{
    commands::{FunctionalCommand, Var},
    utils,
};

const ENV_VARS_TO_IGNORE: [&str; 4] = ["SHELL", "PWD", "TERM", "PATH"]; // NOTE: u may wanna to
// remove path

#[derive(Error, Debug, Diagnostic)]
pub enum EnvError {
    #[error("fiald to set env var: {var}")]
    #[diagnostic(code(commands::run_time_error))]
    FialdToSetEnv { var: String },

    #[error("u jsut pass a path that not exists as file to load env from: {file}")]
    #[diagnostic(code(commands::worng_usege))]
    EnvFileNotExist { file: PathBuf },

    #[error("u jsut tring to remove unexist env var: {var}")]
    #[diagnostic(code(commands::worng_usege))]
    EnvVarNotExists { var: String },
}

#[derive(Debug)]
pub enum EnvActionType {
    LoadFromFile,
    Inject,
    Drop,
}

impl std::str::FromStr for EnvActionType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "load_from_file" => Ok(EnvActionType::LoadFromFile),
            "inject" => Ok(EnvActionType::Inject),
            "drop" => Ok(EnvActionType::Drop),
            _ => Err("env type name must be `load` or `drop`")?,
        }
    }
}

#[derive(knus::Decode, Debug)]
pub struct Env {
    #[knus(arguments)]
    vars: Vec<String>,
    #[knus(type_name)]
    action: EnvActionType,
}

impl FunctionalCommand for Env {
    fn run(&self) -> miette::Result<()> {
        match self.action {
            EnvActionType::Inject => {
                for env in &self.vars {
                    let env_split = env.split("=").collect::<Vec<&str>>();

                    let (key, value) = (env_split.first(), env_split.get(1));

                    if let (Some(key), Some(value)) = (key, value) {
                        let mut value = value.to_string();
                        utils::var::format_string_using_vars(&mut value, vars);

                        unsafe { env::set_var(key, value) };
                    } else {
                        Err(EnvError::FialdToSetEnv { var: env.clone() })?
                    }
                }
            }
            EnvActionType::LoadFromFile => {
                for file in &self.vars {
                    let file_path = PathBuf::from(file);
                    if !file_path.exists() {
                        Err(EnvError::EnvFileNotExist { file: file_path })?
                    }

                    let file_containt =
                        String::from_utf8(fs::read(file).into_diagnostic()?).into_diagnostic()?; // FIXME: map the error for better

                    for line in file_containt.lines() {
                        let env_split = line.split("=").collect::<Vec<&str>>();

                        let (key, value) = (env_split.first(), env_split.get(1));

                        if let (Some(key), Some(value)) = (key, value) {
                            let mut value = value.to_string();
                            utils::var::format_string_using_vars(&mut value, vars);

                            unsafe { env::set_var(key, value) };
                        } else {
                            Err(EnvError::FialdToSetEnv {
                                var: line.to_string().clone(),
                            })?
                        }
                    }
                }
            }
            EnvActionType::Drop => {
                if self.vars.is_empty() {
                    for (var_name, _) in env::vars() {
                        if ENV_VARS_TO_IGNORE.contains(&var_name.as_str()) {
                            continue;
                        };

                        unsafe { env::remove_var(var_name) };
                    }
                    return Ok(());
                }

                for var in &self.vars {
                    if env::var(var).is_err() {
                        Err(EnvError::EnvVarNotExists { var: var.clone() })?
                    }

                    unsafe { env::remove_var(var) };
                }
            }
        }

        Ok(())
    }
}
