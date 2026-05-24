use std::sync::Arc;

use crate::{commands::Command, flow_control::FlowRouter};

#[derive(knus::Decode, Debug)]
pub struct If {
    #[knus(property)]
    var: String,
    #[knus(property)]
    is: Option<String>, // TODO: make this an enum between str and bool
    #[knus(children)]
    files: Vec<Command>,
}

impl FlowRouter for If {
    fn get_flow(&self) -> Vec<Arc<Command>> {
        todo!()
    }
}
