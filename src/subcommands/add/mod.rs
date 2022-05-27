use clap::Subcommand;
use clap::Args;
use crate::subcommands::add::dependency::Dependency;
use crate::subcommands::add::module::Module;

pub(crate) mod dependency;
pub(crate) mod module;

#[derive(Args)]
pub(crate) struct Add {
    #[clap(subcommand)]
    pub(crate) command: AddCommand,
}

#[derive(Subcommand)]
pub(crate) enum AddCommand {
    Module(Module),
    Dependency(Dependency),
}
