use clap::Args;
use knapsac_lib::dependency::Dependency;
use knapsac_lib::registry::Registry;
use std::path::{Path, PathBuf};

#[derive(Args)]
pub(crate) struct StandaloneModule {
    dep_source_file: PathBuf,
}

impl StandaloneModule {
    pub(crate) fn handle_command(&self, depender: &Path) {
        let mut r = Registry::load();
        let dependency = Dependency::Standalone(self.dep_source_file.clone());
        r.add_dependency_to_item(depender, dependency);
    }
}
