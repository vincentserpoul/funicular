use super::env_vars::EnvVars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvironmentVars(pub HashMap<String, String>);

#[derive(Debug, Deserialize, Serialize)]
pub struct Provisioner {
    pub name: String,
    pub script_path: String,
    pub environment_vars: EnvironmentVars,
    pub lbu_additions: Option<Vec<String>>,
}

#[derive(Error, Debug)]
pub enum ProvisionerError {
    #[error("script {name:?} for provisioner {script_path:?} does not exist")]
    ScriptNotExist { name: String, script_path: String },
    #[error("unknown provisioner store error")]
    Unknown,
}

impl Provisioner {
    pub fn run_all_scripts(&self) -> Result<(), ProvisionerError> {
        println!("{:?}", self.script_path);
        Ok(())
    }
}

impl EnvVars for EnvironmentVars {
    fn to_hash_map(&self, existing_key: &str) -> HashMap<String, String> {
        self.0
            .iter()
            .map(|(key, val)| {
                (
                    existing_key.to_owned() + "_" + key.to_uppercase().as_str(),
                    val.to_owned(),
                )
            })
            .collect()
    }
}
