pub mod device;

use crate::config::apk_overlay::APKOverlay;
use crate::docker;
use crate::hardware::Hardware;
use anyhow::Result;
use device::check_path;
use gumdrop::Options;
use std::ffi::OsString;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use thiserror::Error;

// Options accepted for the `build` command
#[derive(Debug, Options)]
pub struct BuildOpts {
    #[options(help = "show this help message")]
    pub help: bool,

    #[options(help = "target directory for the created archives")]
    target_dir: Option<PathBuf>,

    #[options(required, help = "path of your config file")]
    config_file: PathBuf,

    #[options(help = "hardware if you want to create a boot archive: i.e. rpi")]
    hardware: Option<Hardware>,

    #[options(help = "device storage path: i.e. /dev/sda")]
    device_path: Option<PathBuf>,

    #[options(help = "specify no prompt before writing on specify device")]
    force_device_write: Option<bool>,
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("`{0}` is not a toml config file")]
    NotTOMLConfigFile(PathBuf),
}

impl BuildOpts {
    pub fn run(&self) -> Result<()> {
        build(
            &self.config_file,
            self.target_dir.as_ref(),
            self.hardware,
            self.device_path.as_ref(),
            self.force_device_write,
        )?;
        Ok(())
    }
}

pub fn build(
    config_file: &PathBuf,
    target_dir: Option<&PathBuf>,
    hardware: Option<Hardware>,
    device_path: Option<&PathBuf>,
    force_device_write: Option<bool>,
) -> Result<()> {
    if !config_file.is_file() || config_file.extension() != Some(OsString::from("toml").as_os_str())
    {
        return Err(BuildError::NotTOMLConfigFile(config_file.clone()).into());
    }

    if let Some(dp) = device_path {
        check_path(dp)?;
    }

    // config_dir
    let mut config_dir = config_file.clone();
    config_dir.pop();

    // if target_dir is None, use the config dir
    let default_config_dir = config_dir.clone();
    let target_dir = target_dir.clone().unwrap_or(&default_config_dir);

    // create config.env file
    create_config_env_file(config_file, &config_dir)?;

    // build
    docker::run_build(
        &config_dir,
        target_dir,
        hardware,
        device_path,
        force_device_write,
    )?;

    Ok(())
}

fn create_config_env_file(config_file: &PathBuf, config_dir: &PathBuf) -> Result<()> {
    // generate the config.env
    let overlay = APKOverlay::from_path(config_file)?;

    // create config_dir/config.env
    let mut config_file_path = config_dir.clone();
    config_file_path.push("config.env");

    let mut file = File::create(config_file_path)?;

    // Write the config string to `file`, returns `io::Result<()>`
    file.write_all(overlay.to_string().as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_nofile() {
        let curr_path = std::env::current_dir().unwrap();
        let err = build(&curr_path, None, None, None, None).map_err(|e| e.to_string());
        let expected = Err(BuildError::NotTOMLConfigFile(curr_path)).map_err(|e| e.to_string());
        assert_eq!(err, expected);
    }

    #[test]
    fn build_not_toml_file() {
        let mut curr_path = std::env::current_dir().unwrap();
        curr_path.push("config.yaml");
        let err = build(&curr_path, None, None, None, None).map_err(|e| e.to_string());
        let expected = Err(BuildError::NotTOMLConfigFile(curr_path)).map_err(|e| e.to_string());
        assert_eq!(err, expected);
    }
}
