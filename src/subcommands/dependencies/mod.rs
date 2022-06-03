mod add;
mod get;

use crate::subcommands::dependencies::add::Add;
use crate::subcommands::dependencies::get::Get;
use clap::Args;
use clap::Subcommand;
use knapsac_lib::entry::Entry;

#[derive(Args)]
pub(crate) struct Dependencies {
    #[clap(subcommand)]
    command: DependenciesCommand,
}

#[derive(Subcommand)]
pub(crate) enum DependenciesCommand {
    Add(Add),
    Get(Get),
}

impl Dependencies {
    pub(crate) fn handle_command(&self, depender: Entry) {
        match &self.command {
            DependenciesCommand::Add(a) => a.handle_command(depender),
            DependenciesCommand::Get(g) => g.handle_command(depender),
        }
    }
}
