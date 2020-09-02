use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum Hardware {
    RPi,
}

#[derive(Error, Debug)]
pub enum HardwareError {
    #[error("`{0}` is not a valid hardware")]
    NotExists(String),
}

use std::str::FromStr;

impl FromStr for Hardware {
    type Err = HardwareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rpi" => Ok(Hardware::RPi),
            _ => Err(HardwareError::NotExists(s.into())),
        }
    }
}

impl fmt::Display for Hardware {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hardware::RPi => write!(f, "rpi"),
        }
    }
}
