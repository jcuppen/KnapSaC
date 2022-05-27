use crate::subcommands::remove::module::Module;
use clap::Args;
use clap::Subcommand;
pub(crate) mod module;

#[derive(Args)]
pub(crate) struct Remove {
    #[clap(subcommand)]
    pub(crate) command: RemoveCommand,
}

#[derive(Subcommand)]
pub(crate) enum RemoveCommand {
    Module(Module),
}
