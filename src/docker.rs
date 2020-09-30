use super::hardware::Hardware;
use anyhow::Result;
use bollard::container::{Config, CreateContainerOptions, LogsOptions, StartContainerOptions};
use bollard::models::*;
use bollard::Docker;
use futures_util::stream::TryStreamExt;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::runtime::Runtime;

const DOCKER_APKOVL_BUILD_IMG: &str = "vincentserpoul/funicular:v0.1.0";
const DOCKER_APKOVL_CONTAINER_NAME: &str = "funicular";

pub fn run_build(
    config_dir: &PathBuf,
    target_dir: &PathBuf,
    hardware: Option<Hardware>,
    device_path: Option<&PathBuf>,
    force_device_write: Option<bool>,
) -> Result<()> {
    let config_dir_canon = fs::canonicalize(config_dir)?;
    let config_dir_string = config_dir_canon.into_os_string().into_string().unwrap();

    let target_dir_canon = fs::canonicalize(target_dir)?;
    let target_dir_string = target_dir_canon.into_os_string().into_string().unwrap();

    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        #[cfg(unix)]
        let docker = Docker::connect_with_unix_defaults().unwrap();
        #[cfg(windows)]
        let docker = Docker::connect_with_named_pipe_defaults().unwrap();

        let container_options = CreateContainerOptions {
            name: DOCKER_APKOVL_CONTAINER_NAME,
        };

        let mut mounts = vec![
            Mount {
                source: Some(config_dir_string.clone()),
                target: Some(String::from("/apk/config")),
                typ: Some(MountTypeEnum::BIND),
                consistency: Some(String::from("default")),
                ..Default::default()
            },
            Mount {
                source: Some(target_dir_string),
                target: Some(String::from("/target")),
                typ: Some(MountTypeEnum::BIND),
                consistency: Some(String::from("default")),
                ..Default::default()
            },
        ];

        let pp = config_dir_string.clone() + "/provisioners";
        let provisioner_path = Path::new(&pp);
        if provisioner_path.is_dir() {
            mounts.push(Mount {
                source: Some(config_dir_string + "/provisioners"),
                target: Some(String::from("/apk/additional_provisioners")),
                typ: Some(MountTypeEnum::BIND),
                consistency: Some(String::from("default")),
                ..Default::default()
            });
        }

        let host_config = HostConfig {
            privileged: Some(true),
            auto_remove: Some(true),
            mounts: Some(mounts),
            ..Default::default()
        };

        let mut cmd_option: Vec<String> = Vec::new();

        if let Some(h) = hardware {
            cmd_option.push(String::from("-H"));
            cmd_option.push(h.to_string());
        }
        if let Some(d) = device_path {
            cmd_option.push(String::from("-d"));
            let ds = d.clone().into_os_string().into_string().unwrap();
            cmd_option.push(ds);
            if let Some(f) = force_device_write {
                if f {
                    cmd_option.push(String::from("-f"));
                }
            }
        }

        let config = Config {
            image: Some(DOCKER_APKOVL_BUILD_IMG),
            host_config: Some(host_config),
            cmd: Some(cmd_option.iter().map(AsRef::as_ref).collect()),
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

        let log_options = Some(LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: true,
            ..Default::default()
        });

        docker
            .logs(DOCKER_APKOVL_CONTAINER_NAME, log_options)
            .map_err(|e| println!("{}", e.to_string()))
            .map_ok(|x| println!("{}", x.to_string()))
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
    });

    Ok(())
}
