pub(crate) mod add;
pub(crate) mod add_dependency;
pub(crate) mod get_dependency;
pub(crate) mod remove;

use crate::subcommands::executable::add::Add;
use crate::subcommands::executable::add_dependency::AddDependency;
use crate::subcommands::executable::get_dependency::GetDependency;
use crate::subcommands::executable::remove::Remove;
use clap::Args;
use clap::Subcommand;

#[derive(Args)]
pub(crate) struct Executable {
    #[clap(subcommand)]
    pub(crate) command: ExecutableCommand,
}

#[derive(Subcommand)]
pub(crate) enum ExecutableCommand {
    Add(Add),
    AddDependency(AddDependency),
    GetDependency(GetDependency),
    Remove(Remove),
}
