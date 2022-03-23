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
    Dump,
    /// Adds an entry
    Add {
        /// Path to new entry
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
