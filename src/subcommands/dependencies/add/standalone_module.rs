use clap::Args;
use knapsac_lib::registry::Registry;
use knapsac_lib::Dependency;
use knapsac_lib::entry::Entry;

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
        r.add_dependency(depender, self.dependency_identifier.clone(), Dependency::StandaloneModule);
    }
}
