use clap::Args;
use knapsac_lib::registry::Registry;
use knapsac_lib::Dependency;
use std::path::PathBuf;
use knapsac_lib::entry::Entry;

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
        r.add_dependency(depender, self.dependency_identifier.clone(), dependency);
    }
}
