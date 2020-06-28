use anyhow;
use gumdrop::Options;
use std::path::PathBuf;

// Options accepted for the `build` command
#[derive(Debug, Options)]
pub struct BuildOpts {
    #[options(help = "print help message")]
    help: bool,

    #[options(
        help = "target directory for your apkovl (usually same as your config file folder)"
    )]
    dir: Option<PathBuf>,

    #[options(help = "path of your config file")]
    config_file: PathBuf,
}

pub fn build(
    dir: Option<PathBuf>,
    _config_file: PathBuf,
) -> Result<(), anyhow::Error> {
    let _output_dir = dir.unwrap_or(PathBuf::from("./out"));

    Ok(())
}
