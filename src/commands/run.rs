use std::{
    env,
    process::{self, Stdio},
    rc::Rc,
    sync::Mutex,
};

use miette::IntoDiagnostic;

use crate::{commands::FunctionalCommand, execute::ExecutionStuck, utils, var::Vars};

#[derive(Debug, Clone)]
enum RunRuntime {
    /// the runtime should support the `-c` flag
    ViaRunTime(String),
}

impl std::str::FromStr for RunRuntime {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let run_time = s;
        Ok(RunRuntime::ViaRunTime(run_time.to_string()))
    }
}

#[derive(knus::Decode, Debug, Clone)]
pub struct Run {
    #[knus(argument)]
    cmd: String,
    #[knus(type_name)]
    runtime: Option<RunRuntime>,
}

impl FunctionalCommand for Run {
    fn exec(
        &self,
        vars: Vars,
        _execution_stuck: Rc<Mutex<ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        let runtime = match &self.runtime {
            Some(RunRuntime::ViaRunTime(run_time)) => run_time.clone(),
            None => env::var("SHELL").map_err(|_| miette::miette!("SHELL env var not found"))?,
        };

        let mut cmd = self.cmd.clone();
        utils::var::format_string_using_vars(&mut cmd, vars.lock().unwrap());

        process::Command::new(&runtime)
            .arg("-c")
            .arg(&cmd)
            .stdout(Stdio::null())
            .spawn()
            .map_err(|err| match err.kind() {
                std::io::ErrorKind::NotFound => miette::miette!("runtime not found: {}", runtime),
                _ => miette::miette!(err),
            })?
            .wait()
            .into_diagnostic()?;

        Ok(())
    }
}
