use super::env_vars::EnvVars;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use sshkeys::PublicKey;
use std::collections::HashMap;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Deserialize, PartialEq, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    AARCH64,
    ARMHF,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arch::AARCH64 => write!(f, "aarch64"),
            Arch::ARMHF => write!(f, "armhf"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct Base {
    pub arch: Arch,
    pub hostname: String,
    pub alpine: Alpine,
    pub networking: Networking,
    pub ssh: SSH,
    pub users: Users,
}

impl Default for Base {
    fn default() -> Self {
        Self {
            arch: Arch::AARCH64,
            hostname: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(15)
                .map(char::from)
                .collect(),
            alpine: Alpine::default(),
            networking: Networking::default(),
            ssh: SSH {
                authorized_keys: Vec::new(),
            },
            users: Users {
                root_password: thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(15)
                    .map(char::from)
                    .collect(),
                remote_user: thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(15)
                    .map(char::from)
                    .collect(),
                remote_user_password: thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(15)
                    .map(char::from)
                    .collect(),
            },
        }
    }
}

impl EnvVars for Base {
    fn to_hash_map(&self, _existing_key: &str) -> HashMap<String, String> {
        let mut hm = [("BASE_ARCH".to_owned(), self.arch.to_string())]
            .iter()
            .cloned()
            .collect::<HashMap<String, String>>();
        hm.extend(
            [("BASE_HOSTNAME".to_owned(), self.hostname.clone())]
                .iter()
                .cloned()
                .collect::<HashMap<String, String>>(),
        );
        hm.extend(self.alpine.to_hash_map("BASE"));
        hm.extend(self.networking.to_hash_map("BASE"));
        hm.extend(self.ssh.to_hash_map("BASE"));
        hm.extend(self.users.to_hash_map("BASE"));
        hm
    }
}

use std::str::FromStr;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Branch {
    EDGE,
    LatestStable,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Branch::EDGE => write!(f, "edge"),
            Branch::LatestStable => write!(f, "latest-stable"),
        }
    }
}

impl FromStr for Branch {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "edge" => Ok(Branch::EDGE),
            "latest-stable" => Ok(Branch::LatestStable),
            _ => Ok(Branch::LatestStable),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Keymap(String, String);

impl fmt::Display for Keymap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn keymap_string() {
        assert_eq!(
            Keymap(String::from("us"), String::from("us")).to_string(),
            "us us"
        );
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseKeymapError {
    #[error("`{0}` can not be parsed as keymap")]
    TooManyElements(String),
}

impl FromStr for Keymap {
    type Err = ParseKeymapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        if iter.clone().count() != 2 {
            return Err(ParseKeymapError::TooManyElements(s.into()));
        }
        let km1 = iter.next().unwrap();
        let km2 = iter.next().unwrap();

        Ok(Keymap(km1.into(), km2.into()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct Alpine {
    pub mirror: String,
    pub version: Version,
    pub timezone: String,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub branch: Branch,
    #[serde(with = "serde_with::rust::display_fromstr")]
    pub keymap: Keymap,
}

impl Default for Alpine {
    fn default() -> Self {
        Self {
            mirror: String::from("http://dl-cdn.alpinelinux.org/alpine"),
            version: "3.12.0".parse().unwrap(),
            timezone: String::from("Asia/Singapore"),
            branch: Branch::LatestStable,
            keymap: Keymap(String::from("us"), String::from("us")),
        }
    }
}

impl EnvVars for Alpine {
    fn to_hash_map(&self, existing_key: &str) -> HashMap<String, String> {
        [
            (
                existing_key.to_owned() + "_" + "ALPINE_MIRROR",
                self.mirror.clone(),
            ),
            (
                existing_key.to_owned() + "_" + "ALPINE_VERSION",
                self.version.to_string(),
            ),
            (
                existing_key.to_owned() + "_" + "ALPINE_TIMEZONE",
                self.timezone.clone(),
            ),
            (
                existing_key.to_owned() + "_" + "ALPINE_BRANCH",
                // todo: do an enum
                self.branch.to_string(),
            ),
            (
                existing_key.to_owned() + "_" + "ALPINE_KEYMAP",
                // todo: do an enum
                self.keymap.to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Networking {
    pub dns_nameservers: Vec<IpAddr>,
}

impl Default for Networking {
    fn default() -> Self {
        Self {
            dns_nameservers: vec![
                IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
            ],
        }
    }
}

impl EnvVars for Networking {
    fn to_hash_map(&self, existing_key: &str) -> HashMap<String, String> {
        [(
            existing_key.to_owned() + "_" + "NETWORKING_DNS_NAMESERVERS",
            (self
                .dns_nameservers
                .iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>()
                .join(", ")),
        )]
        .iter()
        .cloned()
        .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SSH {
    pub authorized_keys: Vec<PublicKey>,
}

impl EnvVars for SSH {
    fn to_hash_map(&self, existing_key: &str) -> HashMap<String, String> {
        [(
            existing_key.to_owned() + "_" + "SSH_AUTHORIZED_KEYS",
            (self
                .authorized_keys
                .iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>()
                .join(", ")),
        )]
        .iter()
        .cloned()
        .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Users {
    pub root_password: String,
    pub remote_user: String,
    pub remote_user_password: String,
}

impl EnvVars for Users {
    fn to_hash_map(&self, existing_key: &str) -> HashMap<String, String> {
        [
            (
                existing_key.to_owned() + "_" + "USERS_ROOT_PASSWORD",
                self.root_password.clone(),
            ),
            (
                existing_key.to_owned() + "_" + "USERS_REMOTE_USER",
                self.remote_user.clone(),
            ),
            (
                existing_key.to_owned() + "_" + "USERS_REMOTE_USER_PASSWORD",
                self.remote_user_password.clone(),
            ),
        ]
        .iter()
        .cloned()
        .collect()
    }
}
