use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct MarkAsModule {
    source_file: PathBuf,
    identifier: String,
}

impl MarkAsModule {
    pub(crate) fn handle_command(&self) {
        let mut r = Registry::load();
        r.mark_as_module(&self.source_file, self.identifier.clone());
    }
}
