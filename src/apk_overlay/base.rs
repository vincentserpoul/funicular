use chrono_tz::Tz;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use sshkeys::PublicKey;
use std::net::{IpAddr, Ipv4Addr};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Base {
    pub hostname: String,
    pub alpine: Alpine,
    pub networking: Networking,
    pub ssh: SSH,
    pub users: Users,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alpine {
    pub mirror: String,
    pub version: Version,
    pub timezone: Tz,
}

impl Default for Alpine {
    fn default() -> Self {
        Self {
            mirror: String::from("http://dl-cdn.alpinelinux.org/alpine"),
            version: "3.12.0".parse().unwrap(),
            timezone: "Asia/Singapore".parse().unwrap(),
        }
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SSH {
    pub authorized_keys: Vec<PublicKey>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Users {
    pub root_password: String,
    pub remote_user: String,
    pub remote_user_password: String,
}
