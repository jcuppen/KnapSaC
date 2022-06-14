mod add;
mod get;
mod has;

use crate::subcommands::dependencies::add::Add;
use crate::subcommands::dependencies::get::Get;
use crate::subcommands::dependencies::has::Has;
use clap::Args;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct Dependencies {
    source_file: PathBuf,
    #[clap(subcommand)]
    command: DependenciesCommand,
}

#[derive(Subcommand)]
pub(crate) enum DependenciesCommand {
    Add(Add),
    Get(Get),
    Has(Has),
}

impl Dependencies {
    pub(crate) fn handle_command(&self) {
        match &self.command {
            DependenciesCommand::Add(a) => a.handle_command(self.source_file.clone()),
            DependenciesCommand::Has(h) => h.handle_command(&self.source_file),
            DependenciesCommand::Get(g) => g.handle_command(&self.source_file),
        }
    }
}
