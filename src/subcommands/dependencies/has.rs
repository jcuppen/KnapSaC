use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::Path;
use std::process::exit;

#[derive(Args)]
pub(crate) struct Has {
    dependency_identifier: String,
}

impl Has {
    pub(crate) fn handle_command(&self, depender: &Path) {
        let r = Registry::load();
        if !r.has_dependency(depender, &self.dependency_identifier) {
            exit(1);
        }
    }
}
