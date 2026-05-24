use std::{fs::read_to_string, path::PathBuf, sync::Arc};

use knus::{Decode as KdlDecode, span as kdl_span};
use miette::{IntoDiagnostic, Result, SourceSpan};

use crate::{
    commands::Command,
    flow_control::{ControlFlowCommand, FlowRouter},
    thread::ThreadCommand,
};

/// the thread indicator is used to define which thread should process execute in
#[derive(Debug)]
pub enum ThreadIndicator {
    /// make a new thread and execute commands in
    Push,
    /// join the last thread in the threads stuck
    Join,
    /// execute in the main thread
    Main,
}

#[derive(Debug)]
pub struct OrgnizedBlock<F: Fn() -> Vec<Arc<Command>>> {
    pub thread_indicator: ThreadIndicator,
    pub get_flow: F,
    pub gate_span: SourceSpan,
}

impl<F: Fn() -> Vec<Arc<Command>>> OrgnizedBlock<F> {
    pub fn new(thread_indicator: ThreadIndicator, get_flow: F, gate_span: SourceSpan) -> Self {
        Self {
            thread_indicator,
            get_flow,
            gate_span,
        }
    }
}

#[derive(KdlDecode, Debug)]
enum KdlCommand {
    ControlFlow(ControlFlowCommand),
    ThreadCommand(ThreadCommand),
    Command(Command),
}

pub struct Parser {
    pub source_code: String,
}

// Diagnostics: 1. rustc: the type parameter `F` is not constrained by the impl trait, self type, or predicates unconstrained type parameter [E0207]
impl<F: Fn() -> Vec<Arc<Command>>> Parser {
    pub fn parse_and_order_commands(
        &self,
        to_run_script: &PathBuf,
    ) -> Result<Vec<OrgnizedBlock<F>>> {
        let kdl_commands = knus::parse::<Vec<KdlCommand>>(
            to_run_script.to_string_lossy(),
            read_to_string(to_run_script).into_diagnostic()?.as_str(),
        )
        .into_diagnostic()?;

        // prepare the output
        let mut ordered_commands_blocks = Vec::<OrgnizedBlock<F>>::new();

        // match and order
        for kdl_command in kdl_commands {
            match kdl_command {
                KdlCommand::ControlFlow(ref control_flow_command) => {
                    ordered_commands_blocks.push(OrgnizedBlock {
                        thread_indicator: ThreadIndicator::Main,
                        // Diagnostics: 1. rust-analyzer: expected F, found Vec<Arc<Command, Global>, Global> [E0308] 2. rustc: attempted to take value of method `get_flow` on type `&flow_control::ControlFlowCommand` method, not a field [E0615] parser.rs:71:64: use parentheses to call the method: `()` 3. rustc: use parentheses to call the method: `()` [E0615] parser.rs:71:56: original diagnostic
                        get_flow: control_flow_command.get_flow,
                        gate_span: get_gate_span(&self.source_code, &kdl_command),
                    });
                }
                KdlCommand::ThreadCommand(thread_command) => {}
                KdlCommand::Command(command) => todo!(),
            }
        }

        Ok(ordered_commands_blocks)
    }
}

fn get_gate_span(source_code: &String, command: &KdlCommand) -> SourceSpan {
    todo!()
}
