use crate::entry::Entry;
use crate::subcommands::add::Add;
use crate::subcommands::dependencies::Dependencies;
use crate::subcommands::remove::Remove;
use clap::Args;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct Executable {
    source_path: PathBuf,
    #[clap(subcommand)]
    command: ExecutableCommand,
}

#[derive(Subcommand)]
pub(crate) enum ExecutableCommand {
    Add(Add),
    Dependencies(Dependencies),
    Remove(Remove),
}

impl Executable {
    pub(crate) fn handle_command(&self) {
        let entry = Entry::Executable(self.source_path.clone());
        match &self.command {
            ExecutableCommand::Add(a) => a.handle_command(entry),
            ExecutableCommand::Dependencies(d) => d.handle_command(entry),
            ExecutableCommand::Remove(r) => r.handle_command(entry),
        }
    }
}
