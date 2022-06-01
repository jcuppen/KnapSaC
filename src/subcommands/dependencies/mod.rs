mod add;
mod get;

use crate::entry::Entry;
use crate::subcommands::dependencies::add::Add;
use crate::subcommands::dependencies::get::Get;
use clap::Args;
use clap::Subcommand;

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
