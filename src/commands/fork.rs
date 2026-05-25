use std::{rc::Rc, sync::Mutex, thread};

use miette::SourceSpan;

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
        execution_stuck: Rc<Mutex<execute::ExecutionStuck>>,
        command_span: knus::span::LineSpan,
    ) -> miette::Result<()> {
        let commands = self.commands.clone();

        let full_source = execution_stuck.lock().unwrap().source_code.clone();
        let source_path = execution_stuck.lock().unwrap().source_path.clone();

        let _ = thread::spawn(move || -> miette::Result<()> {
            let span = SourceSpan::from(command_span);

            let fork_block_source = if span.offset() + span.len() <= full_source.len() {
                &full_source[span.offset()..span.offset() + span.len()]
            } else {
                &full_source[span.offset()..]
            };

            // modifying the execution stuck at across threads are not permitted
            // so isolate the execution stuck of the mian and child thread
            let thread_execution_stuck = Rc::new(Mutex::new(ExecutionStuck::new(
                commands
                    .into_iter()
                    .map(|cmd| (cmd, command_span))
                    .collect::<Vec<_>>(),
                fork_block_source.to_string(),
                source_path,
            )));

            loop {
                let spanned_command = thread_execution_stuck.lock().unwrap().commands.pop();

                if let Some((command, span)) = spanned_command {
                    command.exec(vars.clone(), Rc::clone(&thread_execution_stuck), span)?
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
