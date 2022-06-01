use crate::entry::Entry;
use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;
use std::process::exit;

#[derive(Args)]
pub(crate) struct Add {
    /// Required for Modules
    output_path: Option<PathBuf>,
}

impl Add {
    pub(crate) fn handle_command(&self, entry: Entry) {
        let mut r = Registry::load();
        match entry {
            Entry::Executable(source_path) => r.add_executable(source_path),
            Entry::StandaloneModule(identifier) => {
                if let Some(out) = &self.output_path {
                    r.add_module(identifier, out.clone())
                } else {
                    println!("<OUTPUT_PATH> is required when adding a module");
                    exit(1)
                }
            }
            Entry::PackageModule => {
                panic!()
            }
        }
    }
}
