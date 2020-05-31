use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WLAN {
    pub ssid: String,
    pub passphrase: String,
}
