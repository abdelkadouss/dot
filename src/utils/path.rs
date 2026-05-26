use std::path::PathBuf;

use miette::Result;

pub trait PathUtils {
    /// replace `~` with the user home directory - panic if the HOME env var is not set and
    /// path contains `~`
    fn home_expand(&self) -> Result<PathBuf>;
}

impl PathUtils for PathBuf {
    fn home_expand(&self) -> Result<PathBuf> {
        Ok(PathBuf::from(self.to_string_lossy().replace(
            "~",
            &std::env::var("HOME").map_err(|_| {
                miette::miette!("cant expand `~` because the HOME env var is not set")
            })?,
        )))
    }
}
