use crate::{commands::Command, flow_control::if_condation::If};

mod if_condation;

#[derive(knus::Decode, Debug)]
pub enum ControlFlowCommand {
    If(If),
}

trait FlowRouter {
    fn get_flow(&self) -> Command;
}

impl FlowRouter for ControlFlowCommand {
    fn get_flow(&self) -> Command {
        match self {
            ControlFlowCommand::If(it) => it.get_flow(),
        }
    }
}
