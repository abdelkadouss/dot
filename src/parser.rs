use miette::SourceSpan;

use crate::{commands::Command, flow_control::ControlFlowCommand, thread::ThreadCommand};

/// the thread indicator is used to define which thread should process execute in
pub enum ThreadIndicator {
    /// make a new thread and execute commands in
    Push,
    /// join the last thread in the threads stuck
    Join,
    /// execute in the main thread
    Main,
}

pub trait ThreadedAndFlowControledCommandBlock {
    /// get which thread should execute the block in.
    fn get_thread_counter(&self) -> ThreadIndicator;
    /// get the commands to execute.
    fn get_commands_block(&self) -> Vec<&Command>;
    /// get the source code of the block as string
    fn get_block_source_code(&self) -> String;
    /// get span of the command that route the flow or change the flow thread, like `if` or `join`
    fn get_block_gate_span(&self) -> SourceSpan;
}

#[derive(Debug)]
pub enum ControlFlowBlock {}

impl ThreadedAndFlowControledCommandBlock for ControlFlowBlock {
    fn get_thread_counter(&self) -> ThreadIndicator {
        todo!()
    }
    fn get_commands_block(&self) -> Vec<&Command> {
        todo!()
    }

    fn get_block_source_code(&self) -> String {
        todo!()
    }

    fn get_block_gate_span(&self) -> SourceSpan {
        todo!()
    }
}

#[derive(knus::Decode, Debug)]
enum KdlCommand {
    ControlFlow(ControlFlowCommand),
    ThreadCommand(ThreadCommand),
    Command(Command),
}

pub struct Parser {
    source_code: String,
}

impl Parser {
    pub fn parse_and_order_commands<T: ThreadedAndFlowControledCommandBlock>(
        commands: Vec<Command>,
    ) -> Vec<T> {
        todo!()
    }
}
