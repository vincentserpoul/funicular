pub mod apk_overlay;

use anyhow::Result;
use apk_overlay::APKOverlay;

fn main() -> Result<(), anyhow::Error> {
    let overlay = APKOverlay::from_path("./examples/myregularhost.yaml");
    dbg!(&overlay);
    Ok(())
}
