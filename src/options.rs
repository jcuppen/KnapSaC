use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
/// I am a program and I work, just pass `-h`
pub(crate) struct Opt {
    /// Add new entry
    #[structopt(short, long)]
    pub(crate) add: PathBuf,

    /// Specify the desired registry file
    #[structopt(short, long)]
    pub(crate) registry_filepath: PathBuf,

    /// Lists the local registry
    #[structopt(short, long)]
    pub(crate) dump_registry: bool,
}

pub(crate) fn get_options() -> Opt {
    Opt::from_args()
}
