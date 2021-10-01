use structopt::StructOpt;

#[derive(StructOpt)]
/// I am a program and I work, just pass `-h`
pub(crate) struct Opt {
    /// Specify the desired packages
    #[structopt(short = "p", long)]
    pub(crate) packages: Vec<String>,
}

pub(crate) fn get_options() -> Opt {
    Opt::from_args()
}
