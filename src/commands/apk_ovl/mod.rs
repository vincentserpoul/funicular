pub mod build;
pub mod config_gen;

use build::BuildOpts;
use config_gen::ConfigGenOpts;
use gumdrop::Options;

// Options accepted for the `build` command
#[derive(Debug, Options)]
pub struct ApkOvlOpts {
    #[options(help = "sub commands related to your apk overlay")]
    help: bool,

    #[options(command)]
    command: Option<ApkOvlCommand>,
}

#[derive(Debug, Options)]
enum ApkOvlCommand {
    #[options(help = "helps you generate your config for your apk overlay")]
    ConfigGen(ConfigGenOpts),

    #[options(help = "build your apk overlay")]
    Build(BuildOpts),
}
