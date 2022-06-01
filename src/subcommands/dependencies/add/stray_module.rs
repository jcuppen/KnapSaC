use crate::entry::Entry;
use clap::Args;
use knapsac_lib::registry::Registry;
use knapsac_lib::Dependency;
use std::path::PathBuf;

#[derive(Args)]
/// Adds a dependency to an executable
///
/// [Examples]
/// The following command:
///
///     knapsac executable /home/my_user/a.c dependencies add stray-module b /home/my_user/b/out/
///
/// will dependencies the dependency registered under identifier 'b' to
/// the executable registered for '/home/my_user/a.c'
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct StrayModule {
    #[clap(verbatim_doc_comment)]
    dependency_identifier: String,
    #[clap(verbatim_doc_comment)]
    dependency_output_path: PathBuf,
}

impl StrayModule {
    pub(crate) fn handle_command(&self, depender: Entry) {
        let mut r = Registry::load();
        let dependency = Dependency::StrayModule(self.dependency_output_path.clone());
        match depender {
            Entry::Executable(source_path) => r.add_dependency_to_executable(
                &source_path,
                self.dependency_identifier.clone(),
                dependency,
            ),
            Entry::StandaloneModule(module_identifier) => r.add_dependency_to_module(
                &module_identifier,
                self.dependency_identifier.clone(),
                dependency,
            ),
            Entry::PackageModule => {
                panic!()
            }
        }
    }
}
