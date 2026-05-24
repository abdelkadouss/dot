use std::sync::Arc;

use crate::{commands::Command, flow_control::if_condation::If};

mod if_condation;

#[derive(knus::Decode, Debug)]
pub enum ControlFlowCommand {
    If(If),
}

pub trait FlowRouter {
    fn get_flow(&self) -> Vec<Arc<Command>>;
}

impl FlowRouter for ControlFlowCommand {
    fn get_flow(&self) -> Vec<Arc<Command>> {
        match self {
            ControlFlowCommand::If(it) => it.get_flow(),
        }
    }
}
