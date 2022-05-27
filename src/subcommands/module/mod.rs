pub(crate) mod add;
pub(crate) mod add_dependency;
pub(crate) mod get;
pub(crate) mod get_dependency;
pub(crate) mod remove;

use crate::subcommands::module::add::Add;
use crate::subcommands::module::add_dependency::AddDependency;
use crate::subcommands::module::get::Get;
use crate::subcommands::module::get_dependency::GetDependency;
use crate::subcommands::module::remove::Remove;
use clap::Args;
use clap::Subcommand;

#[derive(Args)]
pub(crate) struct Module {
    #[clap(subcommand)]
    pub(crate) command: ModuleCommand,
}

#[derive(Subcommand)]
pub(crate) enum ModuleCommand {
    Add(Add),
    AddDependency(AddDependency),
    Get(Get),
    GetDependency(GetDependency),
    Remove(Remove),
}
