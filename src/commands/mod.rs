pub mod apk_ovl;

use anyhow;
use apk_ovl::ApkOvlOpts;
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
            Some(FunicularCommand::ApkOvl(o)) => {
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
    #[options(help = "all things related to apk overlay config and build")]
    ApkOvl(ApkOvlOpts),
}
