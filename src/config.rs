use std::{env, path::PathBuf};

use crate::utils;

use miette::Result;

const DEFAULT_SOURCE_DIR_ENV_VAR_NAME: &str = "DOT_SOURCE_DIR";
const DEFAULT_SOURCE_DIR: &str = "~/.config/new";

#[derive(Debug)]
pub struct Config {
    pub source_dir: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let tamplates_dir = utils::path::expand(PathBuf::from(
            env::var(DEFAULT_SOURCE_DIR_ENV_VAR_NAME).unwrap_or(DEFAULT_SOURCE_DIR.to_string()),
        ))?;

        Ok(Config {
            source_dir: tamplates_dir,
        })
    }
}
