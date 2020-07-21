use anyhow::Result;
use thiserror::Error;

use gumdrop::Options;
use std::path::PathBuf;

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
        println!("questionnaire");
        Ok(())
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
    NotDir(String),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

impl LsOpts {
    pub fn run(&self) -> Result<()> {
        let path = &self
            .system_config_path
            .clone()
            .unwrap_or(PathBuf::from("./"));

        if !path.is_dir() {
            return Err(
                LsError::NotDir(path.to_str().unwrap().to_owned()).into()
            );
        }
        Ok(())
    }
}
