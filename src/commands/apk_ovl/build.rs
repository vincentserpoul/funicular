use crate::config::apk_overlay::APKOverlay;
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
    let mut config_dir = config_file.clone();
    config_dir.pop();

    // if target_dir is None, use the config dir
    let default_config_dir = config_dir.clone();
    let target_dir = target_dir.clone().unwrap_or(&default_config_dir);

    // create config.env file
    create_config_env_file(config_file, &config_dir)?;

    // build
    build_docker(&config_dir, target_dir)?;

    // remove config.env file
    // remove_config_env_file(&config_dir)?;

    Ok(())
}

use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn create_config_env_file(
    config_file: &PathBuf,
    config_dir: &PathBuf,
) -> Result<()> {
    // generate the config.env
    let overlay = APKOverlay::from_path(config_file)?;

    // create config_dir/config.env
    let mut config_file_path = config_dir.clone();
    config_file_path.push("config.env");

    println!("{:?}", config_file_path);
    let mut file = File::create(config_file_path)?;

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    file.write_all(overlay.to_string().as_bytes())?;

    Ok(())
}

fn remove_config_env_file(config_dir: &PathBuf) -> Result<()> {
    let mut config_file_path = config_dir.clone();
    config_file_path.push("config.env");
    fs::remove_file(config_file_path)?;
    Ok(())
}

//RemoveContainerOptions
use bollard::container::{
    Config, CreateContainerOptions, StartContainerOptions, WaitContainerOptions,
};
use bollard::models::*;
use bollard::Docker;
use futures_util::stream::TryStreamExt;

const DOCKER_APKOVL_BUILD_IMG: &'static str =
    "vincentserpoul/funicular-apk:latest";

const DOCKER_APKOVL_CONTAINER_NAME: &'static str = "funicular-apk";

fn build_docker(config_dir: &PathBuf, target_dir: &PathBuf) -> Result<()> {
    let config_dir_string = config_dir
        .to_owned()
        .into_os_string()
        .into_string()
        .unwrap();

    let target_dir_string = target_dir
        .to_owned()
        .into_os_string()
        .into_string()
        .unwrap();

    smol::block_on(async {
        #[cfg(unix)]
        let docker = Docker::connect_with_unix_defaults().unwrap();
        #[cfg(windows)]
        let docker = Docker::connect_with_named_pipe_defaults().unwrap();

        let container_options = CreateContainerOptions {
            name: DOCKER_APKOVL_CONTAINER_NAME,
        };

        let host_config = HostConfig {
            privileged: Some(true),
            mounts: Some(vec![
                Mount {
                    target: Some(String::from("/apk/config")),
                    source: Some(config_dir_string.clone()),
                    _type: Some(MountTypeEnum::BIND),
                    consistency: Some(String::from("default")),
                    ..Default::default()
                },
                Mount {
                    target: Some(String::from("/apk/additional_provisioners")),
                    source: Some(config_dir_string + "/provisioners"),
                    _type: Some(MountTypeEnum::BIND),
                    consistency: Some(String::from("default")),
                    ..Default::default()
                },
                Mount {
                    target: Some(String::from("/apk/target")),
                    source: Some(target_dir_string),
                    _type: Some(MountTypeEnum::BIND),
                    consistency: Some(String::from("default")),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let config = Config {
            image: Some(DOCKER_APKOVL_BUILD_IMG),
            host_config: Some(host_config),
            ..Default::default()
        };

        docker
            .create_container(Some(container_options), config)
            .await
            .unwrap();

        docker
            .start_container(
                DOCKER_APKOVL_CONTAINER_NAME,
                None::<StartContainerOptions<String>>,
            )
            .await
            .unwrap();

        docker
            .wait_container(
                DOCKER_APKOVL_CONTAINER_NAME,
                None::<WaitContainerOptions<String>>,
            )
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        // docker
        //     .remove_container(
        //         DOCKER_APKOVL_CONTAINER_NAME,
        //         None::<RemoveContainerOptions>,
        //     )
        //     .await
        //     .unwrap();
    });

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
