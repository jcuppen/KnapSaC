use knapsac_lib::registry::Registry;
use clap::{ArgEnum, Args};
use knapsac_lib::version::SemVerIncrement;

#[derive(Clone, ArgEnum)]
pub enum VersionIncrement {
    Major,
    Minor,
    Patch,
}

#[derive(Args)]
pub(crate) struct Publish {
    #[clap(arg_enum)]
    version_increment: VersionIncrement,
}

fn vi_to_svi(version_increment: &VersionIncrement) -> SemVerIncrement {
    match version_increment {
        VersionIncrement::Major => SemVerIncrement::Major,
        VersionIncrement::Minor => SemVerIncrement::Minor,
        VersionIncrement::Patch => SemVerIncrement::Patch,
    }
}

impl Publish {
    pub(crate) fn handle_command(&self, identifier: &str) {
        let mut r = Registry::load();
        r.publish(identifier, vi_to_svi(&self.version_increment));
    }
}
