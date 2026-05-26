use std::{fs, path::PathBuf, rc::Rc, sync::Mutex};

use miette::IntoDiagnostic;
use path_absolutize::Absolutize;

use crate::{
    commands::FunctionalCommand,
    execute,
    utils::{self, path::PathUtils},
    var::Vars,
};

#[derive(knus::Decode, Debug, Clone)]
pub struct Rm {
    #[knus(arguments)]
    paths: Vec<String>,
}

impl FunctionalCommand for Rm {
    fn exec(
        &self,
        vars: Vars, // TODO: format the paths using those
        _execution_stuck: Rc<Mutex<execute::ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        for path in &self.paths {
            let mut file = path.clone();

            utils::var::format_string_using_vars(&mut file, vars.lock().unwrap());

            let file = PathBuf::from(&file)
                .home_expand()?
                .absolutize()
                .into_diagnostic()?
                .to_path_buf();

            if !file.exists() {
                Err(miette::miette!("file not found: {}", file.display()))?;
            }

            if file.is_dir() {
                fs::remove_dir_all(file).into_diagnostic()?
            } else {
                fs::remove_file(file).into_diagnostic()?
            }
        }

        Ok(())
    }
}
