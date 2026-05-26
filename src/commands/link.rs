#![allow(dead_code)]
use std::{fs, path::PathBuf, rc::Rc, sync::Mutex};

use miette::IntoDiagnostic;

use crate::{commands::FunctionalCommand, execute::ExecutionStuck, var::Vars};

#[derive(knus::Decode, Debug, Clone)]
pub struct Link {
    #[knus(property)]
    path: String,
    #[knus(property)]
    to: String,
}

impl FunctionalCommand for Link {
    /// make a symbolic link - ***on non-unix platforms do nothing***
    fn exec(
        &self,
        _vars: Vars,
        _execution_stuck: Rc<Mutex<ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        let path = PathBuf::from(&self.path).canonicalize().into_diagnostic()?;
        let to = PathBuf::from(&self.to).canonicalize().into_diagnostic()?;

        if !path.exists() {
            return Err(miette::miette!("file not found: {}", path.display()));
        }

        let to = match to.is_dir() {
            true => to.join(path.file_name().unwrap()),
            false => to,
        };

        if to.exists() && to.is_symlink() {
            fs::remove_file(&to).into_diagnostic()?;
        } else if to.exists() && !to.is_symlink() {
            return Err(miette::miette!(
                "cant make a symlink because target path exists and is not a symlink"
            ));
        }

        #[cfg(unix)]
        std::os::unix::fs::symlink(path, to).into_diagnostic()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_link() {
        let vars = Vars::new(Vec::<crate::var::Var>::new().into());
        let execution_stuck = Rc::new(Mutex::new(ExecutionStuck::empty(
            String::new(),
            std::path::PathBuf::new(),
        )));

        let temp_dir = tempdir().unwrap();
        let temp_dir_path = temp_dir.path();

        fs::create_dir_all(temp_dir_path.join("example")).unwrap();
        fs::write(temp_dir_path.join("example/Dotfile.kdl"), "").unwrap();
        fs::create_dir_all(temp_dir_path.join("target")).unwrap();

        Link {
            path: temp_dir_path
                .join("example/Dotfile.kdl")
                .to_string_lossy()
                .to_string(),
            to: temp_dir_path.join("target").to_string_lossy().to_string(),
        }
        .exec(
            vars.clone(),
            execution_stuck.clone(),
            knus::span::LineSpan::default(),
        )
        .inspect_err(|err| eprintln!("{err}"))
        .unwrap();

        Link {
            path: temp_dir_path.join("example").to_string_lossy().to_string(),
            to: temp_dir_path.join("target").to_string_lossy().to_string(),
        }
        .exec(
            vars.clone(),
            execution_stuck.clone(),
            knus::span::LineSpan::default(),
        )
        .inspect_err(|err| eprintln!("{err}"))
        .unwrap();

        assert!(temp_dir_path.join("target/Dotfile.kdl").exists());
        assert!(temp_dir_path.join("target/example").exists());
        assert!(temp_dir_path.join("target/example/Dotfile.kdl").exists());
    }
}
