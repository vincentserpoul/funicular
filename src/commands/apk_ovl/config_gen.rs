use anyhow;
use gumdrop::Options;

// Options accepted for the `make` command
#[derive(Debug, Options)]
pub struct ConfigGenOpts {
    #[options(help = "helps you generate your config file")]
    help: bool,
}

pub fn config_gen() -> Result<(), anyhow::Error> {
    Ok(())
}
