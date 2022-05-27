use crate::add::Add;
use crate::remove::Remove;

use clap::{Parser, Subcommand};
use crate::get::Get;

#[derive(Parser)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Add(Add),
    Get(Get),
    Remove(Remove),
}
