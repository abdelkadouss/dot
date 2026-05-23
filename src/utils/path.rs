use miette::Result;
use std::path::{Path, PathBuf};

/// Expands a path that may begin with `~` to its absolute form.
///
/// Returns `Err` if the `HOME` environment variable is not set.
///
/// # Platform
/// UNIX only — `~` expansion is not a concept on Windows.
#[cfg(unix)]
pub fn expand(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();

    let stripped = match path.strip_prefix("~") {
        Ok(rest) => rest,
        Err(_) => return Ok(path.to_path_buf()),
    };

    let home = std::env::var_os("HOME")
        .ok_or_else(|| miette::miette!("the HOME env var is not setted!"))?;
    Ok(PathBuf::from(home).join(stripped))
}
