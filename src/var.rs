use std::sync::{Arc, Mutex};

pub type Vars = Arc<Mutex<Vec<Var>>>;

#[derive(Debug)]
pub struct Var {
    pub name: String,
    pub value: VarValue,
}

#[derive(Debug)]
pub enum VarValue {
    Str(String),
    Bool(bool),
}
