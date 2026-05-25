use std::{fs::read_to_string, path::PathBuf};

use miette::{IntoDiagnostic, Result};

use crate::{commands::Command, execute::ExecutionStuck};

/// thin wrapper around the `knus::parse` function
pub struct Parser {
    pub source_path: PathBuf,
    pub source_code: String,
}

impl Parser {
    pub fn new(script_path: &PathBuf) -> Result<Self> {
        Ok(Self {
            source_path: script_path.to_path_buf(),
            source_code: read_to_string(script_path).into_diagnostic()?,
        })
    }

    pub fn parse(&self) -> Result<ExecutionStuck> {
        let kdl_commands =
            knus::parse::<Vec<Command>>(&self.source_path.to_string_lossy(), &self.source_code)
                .into_diagnostic()?;

        Ok(ExecutionStuck {
            commands: kdl_commands,
        })
    }
}
