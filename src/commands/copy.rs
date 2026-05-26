#![allow(dead_code)]
use std::{
    fs,
    path::{self, Path},
    rc::Rc,
    sync::Mutex,
};

use miette::{IntoDiagnostic, Result};

use crate::{commands::FunctionalCommand, execute::ExecutionStuck, var::Vars};

#[derive(knus::Decode, Debug, Clone)]
pub struct Copy {
    #[knus(arguments)]
    from: Vec<String>,
    #[knus(property)]
    to: String,
    #[knus(property, default = false)]
    overwrite: bool,
}

impl FunctionalCommand for Copy {
    fn exec(
        &self,
        _vars: Vars,
        _execution_stuck: Rc<Mutex<ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        for path in &self.from {
            let from = path::absolute(path).into_diagnostic()?;
            let to = path::absolute(&self.to).into_diagnostic()?;

            // if dir, copy set the target as dir/file_name
            let to = if to.is_dir() {
                to.join(from.file_name().unwrap())
            } else {
                to
            };

            // check if the args are valid
            if !from.exists() {
                return Err(miette::miette!("file not found: {}", from.display()));
            }

            if to.exists() && !self.overwrite {
                return Err(miette::miette!("file already exists: {}", to.display()));
            }

            match to.is_dir() {
                true => fs::create_dir_all(&to).into_diagnostic()?,
                false => {
                    if let Some(parent) = to.parent() {
                        fs::create_dir_all(parent).into_diagnostic()?;
                    }
                }
            }

            if from.is_dir() {
                copy_dir_recursively(&from, &to)?;
            } else if from.is_file() {
                fs::copy(from, to).into_diagnostic()?;
            } else {
                return Err(miette::miette!(
                    "the source path is neither a regular file nor a symlink to a regular file"
                ));
            };
        }

        Ok(())
    }
}

fn copy_dir_recursively(source_path: &Path, dest_path: &Path) -> Result<()> {
    if !source_path.exists() || !source_path.is_dir() {
        return Err(miette::miette!(
            "can't copy source recursively because it not an existing directory"
        ));
    }

    fs::create_dir_all(dest_path).into_diagnostic()?;
    for entry in fs::read_dir(source_path).into_diagnostic()? {
        let entry = entry.into_diagnostic()?;
        let dest_entry = dest_path.join(entry.file_name());
        // Recursively copy or move
        if entry.path().is_dir() {
            copy_dir_recursively(&entry.path(), dest_entry.as_path())?;
        } else {
            fs::copy(entry.path(), &dest_entry).into_diagnostic()?;
        }
    }

    Ok(())
}
