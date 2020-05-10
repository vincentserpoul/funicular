use chrono_tz::Tz;
use semver::Version;
use serde_derive::Deserialize;
use serde_yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./examples/regular.yaml")?;
    let d: APKOverlay = serde_yaml::from_reader(f)?;
    dbg!(&d);
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct APKOverlay {
    base: Base,
}

#[derive(Debug, Deserialize)]
pub struct Base {
    hostname: String,
    alpine: Alpine,
    networking: Networking,
    users: Users,
}

#[derive(Debug, Deserialize)]
pub struct Alpine {
    mirror: String,
    version: Version,
    timezone: Tz,
}

#[derive(Debug, Deserialize)]
pub struct Ethernet {
    enable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Wifi {
    enable: bool,
    ssid: String,
    passphrase: String,
}
#[derive(Debug, Deserialize)]
pub struct Networking {
    dns_nameservers: Vec<std::net::IpAddr>,
}

#[derive(Debug, Deserialize)]
pub struct TwoFactorAuth {
    enabled: bool,
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct SSH {
    authorized_keys: Vec<String>,
    two_factor_auth: TwoFactorAuth,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    root_password: String,
    remote_user: String,
    remote_user_password: String,
}
