use crate::entry::Entry;
use clap::Args;
use knapsac_lib::registry::Registry;
use std::process::exit;

#[derive(Args)]
pub(crate) struct Get {
    dependency_identifier: String,
}

impl Get {
    pub(crate) fn handle_command(&self, depender: Entry) {
        let r = Registry::load();

        let opt = match depender {
            Entry::Executable(source_path) => {
                r.get_dependency_for_executable(&source_path, &self.dependency_identifier)
            }
            Entry::StandaloneModule(module_identifier) => {
                r.get_dependency_for_module(&module_identifier, &self.dependency_identifier)
            }
            Entry::PackageModule => {
                panic!()
            }
        };

        if let Some(d) = opt {
            println!("{}", d.output_path.display());
        } else {
            exit(1);
        }
    }
}
