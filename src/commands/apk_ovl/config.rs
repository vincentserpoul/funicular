use anyhow;
use gumdrop::Options;
use std::path::PathBuf;

#[derive(Debug, Options)]
pub struct ConfigOpts {
    #[options(help = "config file related command, gen, list")]
    pub help: bool,

    #[options(command)]
    command: Option<ConfigCommand>,
}

#[derive(Debug, Options)]
pub enum ConfigCommand {
    #[options(help = "help generate your config for your apk overlay")]
    Gen(GenOpts),

    #[options(help = "list all configs in a folder")]
    Ls(LsOpts),
}

impl ConfigOpts {
    pub fn run(&self) -> Result<(), anyhow::Error> {
        match &self.command {
            Some(ConfigCommand::Gen(o)) => {
                if o.help_requested() {
                    println!("{}", GenOpts::usage());
                    return Ok(());
                }
                return o.run();
            }
            Some(ConfigCommand::Ls(o)) => {
                if o.help_requested() {
                    println!("{}", LsOpts::usage());
                    return Ok(());
                }
                return o.run();
            }
            None => {
                println!("{}", ConfigOpts::self_usage(&self));
                println!();
                println!("Available commands:");
                println!("{}", ConfigOpts::command_list().unwrap());
                return Ok(());
            }
        }
    }
}

#[derive(Debug, Options)]
pub struct GenOpts {
    #[options(help = "show this help message")]
    help: bool,

    #[options(help = "path of folder containing your system configs")]
    system_config_path: Option<PathBuf>,
}

impl GenOpts {
    pub fn run(&self) -> Result<(), anyhow::Error> {
        println!("questionnaire");
        Ok(())
    }
}

#[derive(Debug, Options)]
pub struct LsOpts {
    #[options(help = "show this help message")]
    help: bool,

    #[options(help = "path of folder containing your system configs")]
    system_config_path: Option<PathBuf>,
}

impl LsOpts {
    pub fn run(&self) -> Result<(), anyhow::Error> {
        println!("ls");
        Ok(())
    }
}
