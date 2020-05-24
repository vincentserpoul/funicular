use anyhow;
use chrono_tz::Tz;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use serde_yaml;
use sshkeys::PublicKey;
use std::fs::File;
use std::io;
use std::net::{IpAddr, Ipv4Addr};
use std::path::Path;

impl Default for APKOverlay {
    fn default() -> APKOverlay {
        APKOverlay {
            base: Base {
                hostname: String::from("nodef"),
                alpine: Alpine {
                    mirror: String::from("http://dl-cdn.alpinelinux.org/alpine"),
                    version: "3.11.6".parse().unwrap(),
                    timezone: "Asia/Singapore".parse().unwrap(),
                },
                networking: Networking {
                    ethernet: Some(Ethernet { enabled: true }),
                    wlan: Some(WLAN {
                        ssid: String::from(""),
                        passphrase: String::from(""),
                    }),
                    dns_nameservers: vec![
                        IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
                        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
                    ],
                },
                ssh: SSH {
                    authorized_keys: Vec::new(),
                    two_factor_auth: Some(TwoFactorAuth {
                        code: String::from(""),
                    }),
                },
                users: Users {
                    root_password: String::from(""),
                    remote_user: String::from(""),
                    remote_user_password: String::from(""),
                },
            },
        }
    }
}

// pub fn apk_overlay_from_path<P: AsRef<Path>>(p: P) -> Result<APKOverlay, anyhow::Error> {
//     let f = File::open(p)?;
//     apk_overlay_from_reader(f)
// }

// pub fn apk_overlay_from_reader<R: io::Read>(r: R) -> Result<APKOverlay, anyhow::Error> {
//     let ao = serde_yaml::from_reader(r)?;
//     Ok(ao)
// }

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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Base {
    hostname: String,
    alpine: Alpine,
    networking: Networking,
    ssh: SSH,
    users: Users,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alpine {
    mirror: String,
    version: Version,
    timezone: Tz,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Networking {
    ethernet: Option<Ethernet>,
    wlan: Option<WLAN>,
    dns_nameservers: Vec<std::net::IpAddr>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ethernet {
    enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WLAN {
    ssid: String,
    passphrase: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SSH {
    authorized_keys: Vec<PublicKey>,
    two_factor_auth: Option<TwoFactorAuth>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TwoFactorAuth {
    code: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Users {
    root_password: String,
    remote_user: String,
    remote_user_password: String,
}
