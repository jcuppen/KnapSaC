use crate::entry::Entry;
use clap::Args;
use knapsac_lib::registry::Registry;
use knapsac_lib::Dependency;

#[derive(Args)]
/// Adds a dependency to an executable
///
/// [Examples]
/// The following command:
///
///     knapsac executable /home/my_user/a.c dependencies add standalone_module b
///
/// will dependencies the dependency registered under identifier 'b' to
/// the executable registered for '/home/my_user/a.c'
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct StandaloneModule {
    /// Identifier for the dependency
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    ///
    /// [Examples]
    ///     my_a
    ///     "my a"
    ///     my\ a
    #[clap(verbatim_doc_comment)]
    dependency_identifier: String,
}
impl StandaloneModule {
    pub(crate) fn handle_command(&self, depender: Entry) {
        let mut r = Registry::load();
        match depender {
            Entry::Executable(source_path) => r.add_dependency_to_executable(
                &source_path,
                self.dependency_identifier.clone(),
                Dependency::StandaloneModule,
            ),
            Entry::StandaloneModule(module_identifier) => r.add_dependency_to_module(
                &module_identifier,
                self.dependency_identifier.clone(),
                Dependency::StandaloneModule,
            ),
            Entry::PackageModule => {
                panic!()
            }
        }
    }
}
