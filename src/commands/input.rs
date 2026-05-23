use crate::commands::{FunctionalCommand, Var};

#[derive(Debug)]
pub enum InputType {
    Str,
    Bool,
}

impl std::str::FromStr for InputType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "str" => Ok(InputType::Str),
            "bool" => Ok(InputType::Bool),
            _ => Err("input type name must be `str` or `bool`")?,
        }
    }
}

#[derive(knus::Decode, Debug)]
pub struct Input {
    #[knus(argument)]
    var: String,
    #[knus(property)]
    prompt: Option<String>,
    #[knus(property)]
    default: Option<String>,
    #[knus(property)]
    options: Option<String>,
    #[knus(type_name)]
    input_type: Option<InputType>,
}

impl FunctionalCommand for Input {
    fn run(&self) -> miette::Result<()> {
        todo!()
    }
}
