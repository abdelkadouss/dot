use crate::commands::Command;

#[derive(knus::Decode, Debug)]
pub struct If {
    #[knus(property)]
    var: String,
    #[knus(property)]
    is: Option<String>, // TODO: make this an enum between str and bool
    #[knus(children)]
    files: Vec<Command>,
}
