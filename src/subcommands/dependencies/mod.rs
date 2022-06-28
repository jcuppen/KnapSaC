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
        let depender = self.source_file.canonicalize().unwrap();
        match &self.command {
            DependenciesCommand::Add(a) => a.handle_command(&depender),
            DependenciesCommand::Has(h) => h.handle_command(&depender),
            DependenciesCommand::Get(g) => g.handle_command(&depender),
        }
    }
}
