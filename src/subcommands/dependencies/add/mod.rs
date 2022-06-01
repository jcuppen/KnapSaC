mod standalone_module;
mod stray_module;

use crate::entry::Entry;
use crate::subcommands::dependencies::add::standalone_module::StandaloneModule;
use crate::subcommands::dependencies::add::stray_module::StrayModule;
use clap::Args;
use clap::Subcommand;

#[derive(Args)]
pub(crate) struct Add {
    #[clap(subcommand)]
    command: AddCommand,
}

#[derive(Subcommand)]
pub(crate) enum AddCommand {
    StrayModule(StrayModule),
    StandaloneModule(StandaloneModule),
}

impl Add {
    pub(crate) fn handle_command(&self, depender: Entry) {
        match &self.command {
            AddCommand::StrayModule(sm) => sm.handle_command(depender),
            AddCommand::StandaloneModule(sam) => sam.handle_command(depender),
        }
    }
}