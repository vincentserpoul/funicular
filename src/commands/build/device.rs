use anyhow::Result;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("`{0}` is does not seem to exist")]
    NotDir(PathBuf),
}

pub fn check_device(device_path: &Path) -> Result<()> {
    // if path is not a directory
    if !device_path.is_dir() {
        return Err(DeviceError::NotDir(PathBuf::from(device_path)).into());
    }

    Ok(())
}
