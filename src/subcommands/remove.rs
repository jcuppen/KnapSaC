use crate::entry::Entry;
use clap::Args;
use knapsac_lib::registry::Registry;

#[derive(Args)]
pub(crate) struct Remove {}

impl Remove {
    pub(crate) fn handle_command(&self, entry: Entry) {
        let mut r = Registry::load();
        match entry {
            Entry::Executable(source_path) => r.remove_executable(&source_path),
            Entry::StandaloneModule(module_identifier) => r.remove_module(&module_identifier),
            Entry::PackageModule => {
                panic!()
            }
        }
    }
}
