use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
/// I am a program and I work, just pass `-h`
pub(crate) struct Opt {
    /// Specify the desired modules
    #[structopt(short = "m", long)]
    pub(crate) modules: Vec<String>,

    /// Specify the target location
    #[structopt(short = "l", long)]
    pub(crate) location: Option<PathBuf>,
}

pub(crate) fn get_options() -> Opt {
    Opt::from_args()
}
