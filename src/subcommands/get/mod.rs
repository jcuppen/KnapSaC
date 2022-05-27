pub(crate) mod dependency;
pub(crate) mod module;

use clap::Subcommand;
use clap::Args;
use crate::get::dependency::Dependency;
use crate::get::module::Module;

#[derive(Args)]
pub(crate) struct Get {
    #[clap(subcommand)]
    pub(crate) command: GetCommand,
}

#[derive(Subcommand)]
pub(crate) enum GetCommand {
    Dependency(Dependency),
    Module(Module),
}
