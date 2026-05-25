use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

pub type Vars = Arc<Mutex<Vec<Var>>>;

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub value: VarValue,
}

#[derive(Debug, Clone)]
pub enum VarValue {
    Str(String),
    Bool(bool),
}

impl Display for VarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarValue::Str(s) => write!(f, "{}", s),
            VarValue::Bool(b) => write!(f, "{}", b),
        }
    }
}
