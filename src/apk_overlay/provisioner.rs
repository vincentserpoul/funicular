use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Provisioner {
    pub name: String,
    pub scripts: Vec<String>,
    pub environment_vars: HashMap<String, String>,
    pub lbu_additions: Option<Vec<String>>,
}
