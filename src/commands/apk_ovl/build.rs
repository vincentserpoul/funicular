use anyhow::Result;
use gumdrop::Options;
use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;

// Options accepted for the `build` command
#[derive(Debug, Options)]
pub struct BuildOpts {
    #[options(help = "show this help message")]
    pub help: bool,

    #[options(
        help = "target directory for your apkovl (usually same as your config file folder)"
    )]
    target_dir: Option<PathBuf>,

    #[options(required, help = "path of your config file")]
    config_file: PathBuf,
}

#[derive(Error, Debug)]
pub enum BuildError {
    #[error("`{0}` is not a toml config file")]
    NotTOMLConfigFile(PathBuf),
}

impl BuildOpts {
    pub fn run(&self) -> Result<()> {
        build(&self.config_file, *&self.target_dir.as_ref())?;
        Ok(())
    }
}

pub fn build(
    config_file: &PathBuf,
    target_dir: Option<&PathBuf>,
) -> Result<()> {
    if !config_file.is_file()
        || config_file.extension() != Some(OsString::from("toml").as_os_str())
    {
        return Err(BuildError::NotTOMLConfigFile(config_file.clone()).into());
    }

    // config_dir
    let config_file = config_file;
    let mut config_dir = config_file.clone();
    config_dir.pop();

    // if target_dir is None, use the config dir
    let default_config_dir = config_dir.clone();
    let target_dir = target_dir.clone().unwrap_or(&default_config_dir);

    println!("{:?} - {:?} - {:?}", config_dir, config_file, target_dir);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_nofile() {
        let curr_path = std::env::current_dir().unwrap();
        let err = build(&curr_path, None).map_err(|e| e.to_string());
        let expected = Err(BuildError::NotTOMLConfigFile(curr_path))
            .map_err(|e| e.to_string());
        assert_eq!(err, expected);
    }

    #[test]
    fn build_not_toml_file() {
        let mut curr_path = std::env::current_dir().unwrap();
        curr_path.push("config.yaml");
        let err = build(&curr_path, None).map_err(|e| e.to_string());
        let expected = Err(BuildError::NotTOMLConfigFile(curr_path))
            .map_err(|e| e.to_string());
        assert_eq!(err, expected);
    }
}
