pub mod base;
pub mod provisioner;

use anyhow;
use base::Base;
use provisioner::Provisioner;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
use std::io;
use std::path::Path;

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

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<APKOverlay, anyhow::Error> {
        let f = File::open(path)?;
        APKOverlay::from_reader(f)
    }

    pub fn from_reader<R: io::Read>(rdr: R) -> Result<APKOverlay, anyhow::Error> {
        let ao: APKOverlay = serde_yaml::from_reader(rdr)?;
        Ok(ao)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct APKOverlay {
    pub base: Base,
    pub provisioners: Option<Vec<Provisioner>>,
}
