use gumdrop::Options;
use std::path::PathBuf;

// Options accepted for the `make` command
#[derive(Debug, Options)]
struct CreateConfigOpts {
    #[options(help = "print help message")]
    help: bool,

    #[options(
        dir = "target root directory for all your machines folder, a new name following the config hostname will be created inside."
    )]
    dir: PathBuf,
}
