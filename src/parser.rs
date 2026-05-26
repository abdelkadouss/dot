use std::{env, fs::read_to_string, path::PathBuf};

use miette::{IntoDiagnostic, Result};
use path_absolutize::Absolutize;

use crate::{commands::Command, execute::ExecutionStuck, utils::path::PathUtils};

/// thin wrapper around the `knus::parse` function
pub struct Parser {
    pub source_path: PathBuf,
    pub source_code: String,
}

impl Parser {
    pub fn new(script_path: &PathBuf) -> Result<Self> {
        Ok(Self {
            source_path: script_path
                .home_expand()?
                .absolutize()
                .into_diagnostic()
                .map_err(|_| miette::miette!("no valid entry point - cant find {:?}", script_path))?
                .to_path_buf(),
            source_code: read_to_string(script_path).into_diagnostic()?,
        })
    }

    pub fn parse(&self) -> Result<ExecutionStuck> {
        // cd to the script dir
        if self.source_path.exists()
            && let Some(parent) = self.source_path.parent()
        {
            env::set_current_dir(parent).into_diagnostic()?;
        }

        let ast = knus::parse_ast::<knus::span::LineSpan>(
            &self.source_path.to_string_lossy(),
            &self.source_code,
        )
        .map_err(miette::Report::new)?;

        let mut commands = ast
            .nodes
            .iter()
            .map(|node| {
                let span = *node.span();
                let command = knus::decode::node::<Command, knus::span::LineSpan>(node)
                    .map_err(|errs| miette::Report::new(errs.into_iter().next().unwrap()))?;

                Ok((command, span))
            })
            .collect::<Result<Vec<_>>>()?;

        // a stack is a LIFO
        commands.reverse();

        Ok(ExecutionStuck {
            commands,
            source_code: self.source_code.clone(),
            source_path: self.source_path.clone(),
        })
    }
}
