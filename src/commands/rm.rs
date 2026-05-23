use std::{
    fs,
    path::{Path, PathBuf},
};

use miette::{Diagnostic, IntoDiagnostic};
use thiserror::Error;

use crate::commands::{FunctionalCommand, Var};

#[derive(Error, Debug, Diagnostic)]
pub enum RmError {
    #[error("u trying to remove file that not exists {file_path}")]
    FileNotExist { file_path: PathBuf },
}

#[derive(knus::Decode, Debug)]
pub struct Rm {
    #[knus(arguments)]
    files: Vec<String>,
}

impl FunctionalCommand for Rm {
    fn run(&self) -> miette::Result<()> {
        for file in &self.files {
            let file_path = target.join(file);

            if !file_path.exists() {
                Err(RmError::FileNotExist {
                    file_path: file_path.clone(),
                })
                .into_diagnostic()?
            }

            if file_path.is_dir() {
                fs::remove_dir_all(file_path).into_diagnostic()?
            } else {
                fs::remove_file(file_path).into_diagnostic()?
            }
        }

        Ok(())
    }
}
