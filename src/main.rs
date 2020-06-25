#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

pub mod apk_overlay;

use anyhow::Result;
use apk_overlay::APKOverlay;

fn main() -> Result<(), anyhow::Error> {
    let overlay = APKOverlay::from_path("./out/exmaple/config.yaml");
    dbg!(&overlay);
    Ok(())
}
