use std::{
    env,
    process::{self, Stdio},
};

use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

use crate::{
    commands::{FunctionalCommand, Var},
    utils,
};

#[derive(Debug, Diagnostic, Error)]
pub enum RunError {
    #[error("u just pass an empty string to the `run` command")]
    #[diagnostic(code(commands::wrong_usege))]
    EmptyCmd { cmd: String },

    #[error("cmd the u pass are fiald. stderr: {stderr:#?}")]
    #[diagnostic(code(commands::run_time_error))]
    CmdFiald { stderr: String },
}

#[derive(Debug)]
enum RunArgType {
    Shell,
    ViaRunTime(String),
}

impl std::str::FromStr for RunArgType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "shell" => Ok(RunArgType::Shell),
            run_time => Ok(RunArgType::ViaRunTime(run_time.to_string())),
        }
    }
}

#[derive(knus::Decode, Debug)]
pub struct Run {
    #[knus(argument)]
    cmd: String,
    #[knus(type_name)]
    arg_type: Option<RunArgType>,
}

impl FunctionalCommand for Run {
    fn run(&self) -> miette::Result<()> {
        env::set_current_dir(target).into_diagnostic()?;

        let mut cmd = self.cmd.clone();
        utils::var::format_string_using_vars(&mut cmd, vars);

        let cmd_vec = cmd.split(" ").collect::<Vec<&str>>();
        match &self.arg_type {
            Some(RunArgType::Shell) | None => {
                let cmd_executable = cmd_vec.first();

                if let Some(command) = cmd_executable {
                    if !process::Command::new(command)
                        .args(cmd_vec.get(1..).unwrap_or(vec![].as_slice()).iter())
                        .stdout(Stdio::null())
                        .spawn()
                        .into_diagnostic()?
                        .wait()
                        .into_diagnostic()?
                        .success()
                    {
                        Err(RunError::CmdFiald { stderr: todo!() })?;
                    }
                } else {
                    Err(RunError::EmptyCmd { cmd })?;
                };
            }
            Some(RunArgType::ViaRunTime(run_time)) => {
                if !process::Command::new(run_time)
                    .args(cmd_vec.iter())
                    .stdout(Stdio::null())
                    .spawn()
                    .into_diagnostic()?
                    .wait()
                    .into_diagnostic()?
                    .success()
                {
                    Err(RunError::CmdFiald { stderr: todo!() })?;
                };
            }
        }

        Ok(())
    }
}
