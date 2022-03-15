use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
/// I am a program and I work, just pass `-h`
pub(crate) struct Opt {
    /// Specify the desired modules
    // #[structopt(short, long)]
    // pub(crate) modules: Vec<String>,

    /// Specify the target location
    // #[structopt(short, long)]
    // pub(crate) location: Option<PathBuf>,

    /// Specify the desired registry file
    #[structopt(short, long)]
    pub(crate) registry_file: PathBuf,

    /// Lists the local registry
    #[structopt(short, long)]
    pub(crate) dump_registry: bool,
}

pub(crate) fn get_options() -> Opt {
    Opt::from_args()
}
