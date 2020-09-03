use anyhow::Result;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("`{0}` is does not seem to exist")]
    NotDir(PathBuf),
}

pub fn check_path(device_path: &Path) -> Result<()> {
    // if path is not a directory
    if !device_path.is_dir() {
        return Err(Error::NotDir(PathBuf::from(device_path)).into());
    }

    Ok(())
}
