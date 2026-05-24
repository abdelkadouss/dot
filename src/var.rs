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
