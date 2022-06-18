use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct Package {
    package_root: PathBuf,
    identifier: String,
}

impl Package {
    pub(crate) fn handle_command(&self) {
        let mut r = Registry::load();
        r.package(self.identifier.clone(), &self.package_root);
    }
}
