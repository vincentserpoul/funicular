pub mod build;

use build::BuildOpts;
use gumdrop::Options;

// Define options for the program.
#[derive(Debug, Options)]
pub struct FunicularOptions {
    // Options here can be accepted with any command (or none at all),
    // but they must come before the command name.
    #[options(help = "print help message")]
    help: bool,

    // The `command` option will delegate option parsing to the command type,
    // starting at the first free argument.
    #[options(command)]
    command: Option<Command>,
}

// The set of commands and the options each one accepts.
//
// Each variant of a command enum should be a unary tuple variant with only
// one field. This field must implement `Options` and is used to parse arguments
// that are given after the command name.
#[derive(Debug, Options)]
pub enum Command {
    // #[options(help = "create config for an apkovl")]
    // CreateConfig(CreateConfigOpts),
    #[options(help = "build an apkovl")]
    Build(BuildOpts),
}
