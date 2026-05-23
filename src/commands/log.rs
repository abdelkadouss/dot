use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    commands::{FunctionalCommand, Var},
    utils,
};
use owo_colors::{AnsiColors, OwoColorize};

#[derive(knus::Decode, Debug)]
enum LogLevel {
    Error,
    Wron,
    Info,
    Hint,
}

impl std::str::FromStr for LogLevel {
    type Err = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => Ok(LogLevel::Error),
            "worn" => Ok(LogLevel::Wron),
            "info" => Ok(LogLevel::Info),
            "hint" => Ok(LogLevel::Hint),
            _ => Err("log type name must be `error`, `worn`, `info` or `hint`")?,
        }
    }
}

#[derive(knus::Decode, Debug)]
pub struct Log {
    #[knus(argument)]
    msg: String,
    #[knus(type_name)]
    level: LogLevel,
}

impl FunctionalCommand for Log {
    fn run(&self) -> miette::Result<()> {
        let (signs, prefix, color) = match self.level {
            LogLevel::Error => ('🚨', "ERROR", AnsiColors::Red),
            LogLevel::Wron => ('🚧', "WRON", AnsiColors::Yellow),
            LogLevel::Info => ('📎', "INFO", AnsiColors::Blue),
            LogLevel::Hint => ('💡', "HINT", AnsiColors::Cyan),
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos(); // FIXME: make this more readable

        let mut msg = self.msg.clone();
        utils::var::format_string_using_vars(&mut msg, vars);

        println!(
            "{}",
            format!("{}|{}{}|: {}", timestamp, signs, prefix.to_uppercase(), msg).color(color)
        );

        Ok(())
    }
}
