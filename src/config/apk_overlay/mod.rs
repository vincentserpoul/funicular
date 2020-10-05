pub mod base;
pub mod env_vars;
pub mod provisioner;

use base::Base;
use color_eyre::eyre::Result;
use env_vars::EnvVars;
use provisioner::Provisioner;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct APKOverlay {
    pub base: Base,
    pub provisioners: Option<Vec<Provisioner>>,
}

impl Default for APKOverlay {
    fn default() -> APKOverlay {
        APKOverlay {
            base: Base::default(),
            provisioners: None,
        }
    }
}

impl APKOverlay {
    pub fn new() -> APKOverlay {
        APKOverlay::default()
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<APKOverlay> {
        let f = File::open(path)?;
        APKOverlay::from_reader(f)
    }

    pub fn from_reader<R: io::Read>(mut rdr: R) -> Result<APKOverlay> {
        let mut buffer = String::new();
        rdr.read_to_string(&mut buffer)?;
        APKOverlay::from_str(buffer.as_str())
    }

    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let mut hm: HashMap<String, String> = HashMap::new();
        hm.extend::<HashMap<String, String>>(self.base.to_hash_map(""));

        if let Some(provisioners) = &self.provisioners {
            let mut ps: Vec<&str> = Vec::new();
            provisioners.iter().for_each(|p| {
                // provisioners'list
                ps.push(p.name.as_str());
                // provisioner
                hm.extend::<HashMap<String, String>>(p.environment_vars.to_hash_map(
                    ("provisioner".to_uppercase() + "_" + p.name.to_uppercase().as_str()).as_str(),
                ));
            });

            // provisioners'list
            if !ps.is_empty() {
                hm.extend::<HashMap<String, String>>(
                    [("provisioners".to_uppercase(), ps.join(" "))]
                        .iter()
                        .cloned()
                        .collect(),
                );
            }
        }

        hm
    }
}

use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorM {
    #[error("invalid toml string")]
    NotValidTOML,
}

use std::str::FromStr;

impl FromStr for APKOverlay {
    type Err = color_eyre::eyre::Error;
    fn from_str(s: &str) -> color_eyre::eyre::Result<Self> {
        let ao: APKOverlay = toml::from_str(s)?;
        Ok(ao)
    }
}

impl fmt::Display for APKOverlay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self
            .to_hash_map()
            .iter()
            .map(|(k, v)| k.clone() + "=\"" + v + "\"\n")
            .collect::<String>();
        write!(f, "{}", s)
    }
}
