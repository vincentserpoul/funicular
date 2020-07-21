pub mod build;
pub mod config;

use build::BuildOpts;
use config::ConfigOpts;
use gumdrop::Options;

// Options accepted for the `build` command
#[derive(Debug, Options)]
pub struct ApkOvlOpts {
    #[options(help = "show this help message")]
    pub help: bool,

    #[options(command)]
    command: Option<ApkOvlCommand>,
}

#[derive(Debug, Options)]
enum ApkOvlCommand {
    #[options(help = "apk overlay config")]
    Config(ConfigOpts),

    #[options(help = "build your apk overlay")]
    Build(BuildOpts),
}

impl ApkOvlOpts {
    pub fn run(&self) -> Result<(), anyhow::Error> {
        match &self.command {
            Some(ApkOvlCommand::Config(o)) => {
                return o.run();
            }
            Some(ApkOvlCommand::Build(o)) => {
                return o.run();
            }
            None => {
                println!("{}", ApkOvlOpts::usage());
                println!();
                println!("Available commands:");
                println!("{}", ApkOvlOpts::command_list().unwrap());
                return Ok(());
            }
        }
    }
}
