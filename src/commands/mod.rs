pub mod build;
pub mod config;

use anyhow;
use build::BuildOpts;
use config::ConfigOpts;

use gumdrop::Options;

// Define options for the program.
#[derive(Debug, Options)]
pub struct FunicularOpts {
    #[options(help = "show this help message")]
    pub help: bool,

    #[options(command)]
    command: Option<FunicularCommand>,
}

impl FunicularOpts {
    pub fn run(&self) -> Result<(), anyhow::Error> {
        match &self.command {
            Some(FunicularCommand::Config(o)) => {
                return o.run();
            }
            Some(FunicularCommand::Build(o)) => {
                return o.run();
            }
            None => {
                println!("{}", FunicularOpts::usage());
                println!();
                println!("Available commands:");
                println!("{}", FunicularOpts::command_list().unwrap());
                return Ok(());
            }
        }
    }
}

// Options accepted for the `funicular` command
#[derive(Debug, Options)]
pub enum FunicularCommand {
    #[options(help = "apk overlay config")]
    Config(ConfigOpts),

    #[options(help = "build your apk overlay")]
    Build(BuildOpts),
}
