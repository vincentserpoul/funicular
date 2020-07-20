pub mod base;
pub mod env_vars;
pub mod provisioner;

use anyhow;
use base::Base;
use env_vars::EnvVars;
use provisioner::Provisioner;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug, Deserialize)]
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

    pub fn from_path<P: AsRef<Path>>(
        path: P,
    ) -> Result<APKOverlay, anyhow::Error> {
        let f = File::open(path)?;
        APKOverlay::from_reader(f)
    }

    pub fn from_reader<R: io::Read>(
        mut rdr: R,
    ) -> Result<APKOverlay, anyhow::Error> {
        let mut s = String::from("");
        &rdr.read_to_string(&mut s)?;
        APKOverlay::from_str(s.as_str())
    }

    pub fn from_str(s: &str) -> Result<APKOverlay, anyhow::Error> {
        let ao: APKOverlay = toml::from_str(s)?;
        Ok(ao)
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
                hm.extend::<HashMap<String, String>>(
                    p.environment_vars.to_hash_map(
                        ("provisioner".to_uppercase()
                            + "_"
                            + p.name.to_uppercase().as_str())
                        .as_str(),
                    ),
                );
            });

            // provisioners'list
            if ps.len() > 0 {
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

    pub fn to_string(&self) -> String {
        self.to_hash_map()
            .iter()
            .map(|(k, v)| k.clone() + "=\"" + v + "\"\n")
            .collect::<String>()
    }
}