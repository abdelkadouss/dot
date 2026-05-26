use std::{path, rc::Rc, sync::Mutex};

use miette::IntoDiagnostic;

use crate::{
    execute::{self, ExecutionStuck},
    utils,
    var::Vars,
};

#[derive(knus::Decode, Debug, Clone)]
pub struct Call {
    #[knus(property)]
    path: String,
}

impl crate::commands::FunctionalCommand for Call {
    fn exec(
        &self,
        vars: Vars,
        _execution_stuck: Rc<Mutex<ExecutionStuck>>,
        _command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        let mut path = self.path.clone();

        utils::var::format_string_using_vars(&mut path, vars.lock().unwrap());

        let path = path::absolute(&path).into_diagnostic()?;

        execute::Execute::execute_script(path)?;

        Ok(())
    }
}
