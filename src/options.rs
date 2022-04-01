use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
/// I am a program and I work, just pass `-h`
pub(crate) struct Opt {
    #[structopt(subcommand)]
    pub(crate) command: Command,

    /// Specify the desired registry file
    #[structopt(short = "r", long)]
    pub(crate) registry_path: PathBuf,
}

#[derive(StructOpt)]
pub(crate) enum Command {
    /// Lists the local registry
    Dump {
        /// Path to specific entry to dump (omit to dump all)
        #[structopt(short, long)]
        entry: Option<PathBuf>,

        /// Specify whether or not to list dependencies
        #[structopt(short = "d")]
        list_dependencies: bool,
    },
    /// Adds an entry
    Add {
        /// Path to new entry
        path: PathBuf,
    },
    /// Adds a new dependency to the provided entry
    AddDependency {
        #[structopt(short = "v")]
        value: i32,
        /// Path to entry where the dependency should be added
        path: PathBuf,
    },
    /// Remove a dependency for the provided entry
    RemoveDependency {
        #[structopt(short = "v")]
        value: i32,
        /// Path to entry where the dependency should be removed
        path: PathBuf,
    },
    /// Removes an entry
    Remove {
        /// Path for entry that needs to be removed
        path: PathBuf,
    },
}

pub(crate) fn get_options() -> Opt {
    Opt::from_args()
}
