#[derive(knus::Decode, Debug)]
pub enum ThreadCommand {
    Fork,
    Join,
}
