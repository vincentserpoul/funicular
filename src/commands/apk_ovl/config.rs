use anyhow::Result;
use thiserror::Error;

use gumdrop::Options;
use std::path::{Path, PathBuf};

#[derive(Debug, Options)]
pub struct ConfigOpts {
    #[options(help = "config file related command, gen, list")]
    pub help: bool,

    #[options(command)]
    command: Option<ConfigCommand>,
}

#[derive(Debug, Options)]
pub enum ConfigCommand {
    #[options(help = "help generate your config for your apk overlay")]
    Gen(GenOpts),

    #[options(help = "list all configs in a folder")]
    Ls(LsOpts),
}

impl ConfigOpts {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Some(ConfigCommand::Gen(o)) => {
                return o.run();
            }
            Some(ConfigCommand::Ls(o)) => {
                return o.run();
            }
            None => {
                println!("{}", ConfigOpts::usage());
                println!();
                println!("Available commands:");
                println!("{}", ConfigOpts::command_list().unwrap());
                return Ok(());
            }
        }
    }
}

#[derive(Debug, Options)]
pub struct GenOpts {
    #[options(help = "show this help message")]
    help: bool,

    #[options(help = "path of folder containing your system configs")]
    system_config_path: Option<PathBuf>,
}

impl GenOpts {
    pub fn run(&self) -> Result<()> {
        todo!();
    }
}

#[derive(Debug, Options)]
pub struct LsOpts {
    #[options(help = "show this help message")]
    help: bool,

    #[options(help = "path of folder containing your system configs")]
    system_config_path: Option<PathBuf>,
}

#[derive(Error, Debug)]
pub enum LsError {
    #[error("`{0}` is not a directory")]
    NotDir(PathBuf),
    #[error("`{0}` doesn't contain any system config yet")]
    NoConfig(PathBuf),
}

impl LsOpts {
    pub fn run(&self) -> Result<()> {
        let system_config_path = &self
            .system_config_path
            .clone()
            .unwrap_or(PathBuf::from("./"));
        let systems = ls(system_config_path)?;
        if systems.len() == 0 {
            return Err(LsError::NoConfig(system_config_path.clone()).into());
        }
        println!("{:?}", systems);

        Ok(())
    }
}

pub fn ls(system_config_path: &Path) -> Result<Vec<String>> {
    // if path is not a directory
    if !system_config_path.is_dir() {
        return Err(LsError::NotDir(PathBuf::from(system_config_path)).into());
    }

    // check subfolders to see which ones fits as a system config
    let systems = system_config_path
        .read_dir()?
        .filter_map(|subdir_entry| {
            let subdir_entry = subdir_entry.unwrap();
            if !subdir_entry.path().is_dir() {
                return None;
            }

            let is_config = subdir_entry
                .path()
                .read_dir()
                .unwrap()
                .find(|config_entry| {
                    if let Ok(config_entry) = config_entry {
                        let path = config_entry.path();
                        if !path.is_dir()
                            && path.file_name().unwrap() == "config.toml"
                        {
                            return true;
                        }
                    }

                    return false;
                })
                .is_some();

            if is_config {
                return Some(subdir_entry.path().display().to_string());
            }
            None
        })
        .collect::<Vec<String>>();

    Ok(systems)
}

#[cfg(test)]
mod tests {

    use super::ls;

    #[test]
    fn ls_out() {
        let mut curr_path = std::env::current_dir().unwrap();
        curr_path.push("out");
        let lss = ls(&curr_path);
        assert!(lss.is_ok());
        let v = lss.unwrap();
        assert_eq!(v.len(), 1);
        assert!(v[0].ends_with("out/example"));
    }
}
