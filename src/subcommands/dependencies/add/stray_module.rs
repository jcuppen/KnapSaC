use clap::Args;
use knapsac_lib::dependency::Dependency;
use knapsac_lib::registry::Registry;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub(crate) struct StrayModule {
    dependency_identifier: String,
    dependency_output_path: PathBuf,
}

impl StrayModule {
    pub(crate) fn handle_command(&self, depender: &Path) {
        let mut r = Registry::load();
        let dependency = Dependency::Stray(
            self.dependency_identifier.clone(),
            self.dependency_output_path.clone(),
        );
        r.add_dependency_to_item(depender, dependency);
    }
}
