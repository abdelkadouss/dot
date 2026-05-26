use std::{env, path::PathBuf};

use miette::{IntoDiagnostic, Result};
use path_absolutize::Absolutize;

use crate::utils::path::PathUtils;

pub const DEFAULT_SOURCE_DIR_ENV_VAR_NAME: &str = "DOT_SOURCE_DIR";
pub const DEFAULT_SOURCE_DIR: &str = "~/.dotfiles";

#[derive(Debug)]
pub struct Config {
    pub source_dir: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let tamplates_dir = PathBuf::from(
            env::var(DEFAULT_SOURCE_DIR_ENV_VAR_NAME).unwrap_or(DEFAULT_SOURCE_DIR.to_string()),
        )
        .home_expand()?
        .absolutize()
        .into_diagnostic()?
        .to_path_buf();

        Ok(Config {
            source_dir: tamplates_dir,
        })
    }
}
