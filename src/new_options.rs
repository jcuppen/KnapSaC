use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::{Add, Initialize, Remove};
use crate::subcommands::add_dependency::AddDependency;

#[derive(Parser)]
pub(crate) struct Cli {
    /// Specify the desired registry file.
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    /// Paths with environment variables are allowed
    /// Path has to point to a valid existing JSON file
    /// If the environment variable KNAPSAC_REGISTRY_PATH is set and this option is omitted $KNAPSAC_REGISTRY_PATH will be used instead
    ///
    /// [Examples]
    ///     ./knapsac -r /home/my_user/registry.json <SUBCOMMAND>
    ///     ./knapsac -r /home/my\ user/registry.json <SUBCOMMAND>
    ///     ./knapsac -r "/home/my user/registry.json" <SUBCOMMAND>
    ///     ./knapsac -r $HOME/registry.json <SUBCOMMAND>
    ///     ./knapsac <SUBCOMMAND> (only allowed when KNAPSAC_REGISTRY_PATH is set)
    ///
    /// [Caveats]
    /// Relative paths might work but are not supported
    ///
    /// [Environment]
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    #[clap(env = "KNAPSAC_REGISTRY_PATH")]
    #[clap(verbatim_doc_comment)]
    pub(crate) registry_path: PathBuf,

    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    Add(Add),
    Remove(Remove),
    Initialize(Initialize),
    AddDependency(AddDependency)
}
