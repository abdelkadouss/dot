use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Mutex,
};

use miette::{Diagnostic, NamedSource, Result, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("executing command: {msg}")]
pub struct CommandError {
    #[source_code]
    src: NamedSource<String>,

    #[label("{label}")]
    span: SourceSpan,

    msg: String,

    label: String,

    #[help]
    help: Option<String>,
}

impl CommandError {
    pub fn new(
        source_path: &Path,
        source_code: &str,
        span: knus::span::LineSpan,
        message: impl Into<String>,
        label: impl Into<String>,
        help: Option<String>,
    ) -> Self {
        let source_span = SourceSpan::from(span);
        Self {
            src: NamedSource::new(source_path.to_string_lossy(), source_code.to_string()),
            span: source_span,
            msg: message.into(),
            label: label.into(),
            help,
        }
    }
}

use crate::{
    commands::{Command, FunctionalCommand},
    parser::Parser,
    var::{Var, Vars},
};

#[derive(Debug, Clone)]
pub struct ExecutionStuck {
    pub commands: Vec<(Command, knus::span::LineSpan)>,
    pub source_code: String,
    pub source_path: PathBuf,
}

pub struct Execute {}

impl Execute {
    pub fn execute_script(path_to_script: PathBuf) -> Result<()> {
        // make the vars shared state
        let vars = Vars::new(Vec::<Var>::new().into());

        let parser = Parser::new(&path_to_script)?;

        let execution_stuck = Rc::new(Mutex::new(parser.parse()?));

        loop {
            let spanned_command = execution_stuck.lock().unwrap().commands.pop();

            if let Some((command, span)) = spanned_command {
                command
                    .exec(vars.clone(), Rc::clone(&execution_stuck), span)
                    .map_err(|e| {
                        // Extract source code from execution_stuck
                        let source = execution_stuck.lock().unwrap().source_code.clone();
                        CommandError::new(
                            &execution_stuck.lock().unwrap().source_path,
                            &source,
                            span,
                            e.to_string(),
                            "here",
                            None,
                        )
                    })?
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl ExecutionStuck {
    /// make a new execution stuck
    pub fn new(
        commands: Vec<(Command, knus::span::LineSpan)>,
        source_code: String,
        source_path: PathBuf,
    ) -> Self {
        Self {
            commands,
            source_code,
            source_path,
        }
    }

    /// return an empty execution stuck
    #[allow(dead_code)]
    pub fn empty(source_code: String, source_path: PathBuf) -> Self {
        Self {
            commands: Vec::<(Command, knus::span::LineSpan)>::new(),
            source_code,
            source_path,
        }
    }
}
