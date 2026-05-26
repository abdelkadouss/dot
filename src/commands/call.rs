use std::{path::PathBuf, rc::Rc, sync::Mutex};

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

        execute::Execute::execute_script(PathBuf::from(&path))?;

        Ok(())
    }
}
