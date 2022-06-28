mod create;
mod publish;
mod upload;
mod build;

use clap::Subcommand;
use clap::Args;
// use crate::subcommands::package::build::Build;
use crate::subcommands::package::create::Create;
use crate::subcommands::package::publish::Publish;
use crate::subcommands::package::upload::Upload;

#[derive(Args)]
pub(crate) struct Package {
    identifier: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Create(Create),
    Publish(Publish),
    Upload(Upload),
    // Build(Build),
}

impl Package {
    pub(crate) fn handle_command(&self) {
        match &self.command {
            Command::Create(c) => c.handle_command(&self.identifier),
            Command::Publish(p) => p.handle_command(&self.identifier),
            Command::Upload(u) => u.handle_command(&self.identifier),
            // Command::Build(b) => b.handle_command(&self.identifier),
        }
    }
}
