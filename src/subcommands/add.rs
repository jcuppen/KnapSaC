use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct Add {
    source_file: PathBuf,
    output_directory: PathBuf,
}

impl Add {
    pub(crate) fn handle_command(&self) {
        let mut r = Registry::load();
        r.add_item(self.source_file.clone(), self.output_directory.clone());
    }
}
