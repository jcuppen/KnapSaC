use crate::subcommands::executable::Executable;
use crate::subcommands::module::Module;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Module(Module),
    Executable(Executable),
}
