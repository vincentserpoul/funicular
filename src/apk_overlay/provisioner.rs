use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Provisioner {
    pub name: String,
    pub script_path: String,
    pub environment_vars: HashMap<String, String>,
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
