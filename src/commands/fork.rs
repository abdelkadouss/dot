use std::{rc::Rc, sync::Mutex, thread};

use crate::{
    commands::{Command, FunctionalCommand},
    execute::{self, ExecutionStuck},
};

#[derive(knus::Decode, Debug, Clone)]
pub struct Fork {
    #[knus(children)]
    commands: Vec<Command>,
}

impl FunctionalCommand for Fork {
    fn exec(
        &self,
        vars: crate::var::Vars,
        _execution_stuck: Rc<Mutex<execute::ExecutionStuck>>,
    ) -> miette::Result<()> {
        let commands = self.commands.clone();

        let _ = thread::spawn(move || -> miette::Result<()> {
            // modifying the execution stuck at across threads are not permitted
            // so isolate the execution stuck of the mian and child thread
            let thread_execution_stuck = Rc::new(Mutex::new(ExecutionStuck::new(commands)));

            loop {
                let command = thread_execution_stuck.lock().unwrap().commands.pop();

                if let Some(cmd) = command {
                    cmd.exec(vars.clone(), Rc::clone(&thread_execution_stuck))?
                } else {
                    break;
                }
            }

            Ok(())
        });

        // TODO: return the thread result
        Ok(())
    }
}
