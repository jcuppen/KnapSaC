use crate::subcommands::add::Add;
use crate::subcommands::dependencies::Dependencies;
use crate::subcommands::get::Get;
use crate::subcommands::has::Has;
use crate::subcommands::mark_as_module::MarkAsModule;
use crate::subcommands::remove::Remove;
use crate::subcommands::search::Search;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Add(Add),
    Has(Has),
    Get(Get),
    Search(Search),
    MarkAsModule(MarkAsModule),
    Remove(Remove),
    Dependencies(Dependencies),
}
