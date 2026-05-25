#![allow(dead_code)]
use std::{fs, rc::Rc, sync::Mutex};

use miette::IntoDiagnostic;

use crate::{commands::FunctionalCommand, execute::ExecutionStuck, utils, var::Vars};

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
    fn exec(&self, _vars: Vars, _execution_stuck: Rc<Mutex<ExecutionStuck>>) -> miette::Result<()> {
        for path in &self.from {
            let from = utils::path::expand(path)?;
            let to = utils::path::expand(&self.to)?;

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
                false => fs::create_dir_all(to.parent().unwrap()).into_diagnostic()?,
            }

            fs::copy(from, to).map(|_| ()).into_diagnostic()?
        }

        Ok(())
    }
}
