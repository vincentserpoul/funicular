pub mod apk_ovl;
use apk_ovl::ApkOvlOpts;
use gumdrop::Options;

// Define options for the program.
#[derive(Debug, Options)]
pub struct FunicularOpts {
    #[options(help = "print help message")]
    help: bool,

    #[options(command)]
    command: Option<FunicularCommand>,
}

// Options accepted for the `build` command
#[derive(Debug, Options)]
pub enum FunicularCommand {
    ApkOvl(ApkOvlOpts),
}
