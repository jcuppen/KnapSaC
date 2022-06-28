use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct Remove {
    source_file: PathBuf,
}

impl Remove {
    pub(crate) fn handle_command(&self) {
        let mut r = Registry::load();
        r.remove_item(&self.source_file.canonicalize().unwrap());
    }
}
