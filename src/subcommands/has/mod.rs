mod by_id;
mod by_source;

use crate::subcommands::has::by_id::ById;
use crate::subcommands::has::by_source::BySource;
use clap::Args;
use clap::Subcommand;

#[derive(Args)]
pub(crate) struct Has {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    ById(ById),
    BySource(BySource),
}

impl Has {
    pub(crate) fn handle_command(&self) {
        match &self.command {
            Command::ById(i) => i.handle_command(),
            Command::BySource(s) => s.handle_command(),
        }
    }
}
