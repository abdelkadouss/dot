#[derive(knus::Decode, Debug)]
pub struct If {
    #[knus(property)]
    var: String,
    #[knus(property)]
    is: Option<String>, // TODO: make this an enum between str and bool
    #[knus(children)]
    files: Vec<Command>,
}

impl std::str::FromStr for IfActionType {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "include" => Ok(IfActionType::Include),
            "exclude" => Ok(IfActionType::Exclude),
            _ => Err("if type name must be `include` or `exclude`")?,
        }
    }
}
