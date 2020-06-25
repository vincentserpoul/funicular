use super::env_vars::EnvVars;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use sshkeys::PublicKey;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Deserialize, Serialize)]
pub struct Base {
    pub hostname: String,
    pub alpine: Alpine,
    pub networking: Networking,
    pub ssh: SSH,
    pub users: Users,
}

impl Default for Base {
    fn default() -> Self {
        Self {
            hostname: String::from(""),
            alpine: Alpine::default(),
            networking: Networking::default(),
            ssh: SSH {
                authorized_keys: Vec::new(),
            },
            users: Users {
                root_password: String::from(""),
                remote_user: String::from(""),
                remote_user_password: String::from(""),
            },
        }
    }
}

impl EnvVars for Base {
    fn to_hash_map(&self, _existing_key: &str) -> HashMap<String, String> {
        let mut hm = [("BASE_HOSTNAME".to_owned(), self.hostname.clone())]
            .iter()
            .cloned()
            .collect::<HashMap<String, String>>();
        hm.extend::<HashMap<String, String>>(self.alpine.to_hash_map("BASE"));
        hm.extend(self.networking.to_hash_map("BASE"));
        hm.extend(self.ssh.to_hash_map("BASE"));
        hm.extend(self.users.to_hash_map("BASE"));
        hm
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alpine {
    pub mirror: String,
    pub version: Version,
    pub timezone: String,
}

impl Default for Alpine {
    fn default() -> Self {
        Self {
            mirror: String::from("http://dl-cdn.alpinelinux.org/alpine"),
            version: "3.12.0".parse().unwrap(),
            timezone: String::from("Asia/Singapore"),
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
            (String::from("'")
                + self
                    .dns_nameservers
                    .iter()
                    .map(|ip| ip.to_string())
                    .collect::<Vec<String>>()
                    .join("', '")
                    .as_str()
                + "'"),
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
            (String::from("'")
                + self
                    .authorized_keys
                    .iter()
                    .map(|ip| ip.to_string())
                    .collect::<Vec<String>>()
                    .join("', '")
                    .as_str()
                + "'"),
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
