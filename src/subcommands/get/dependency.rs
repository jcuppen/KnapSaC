use std::process::exit;
use clap::Args;
use knapsac_lib2::registry::Registry;

#[derive(Args)]
/// Gets a dependency for a module based on the module's manifest
///
/// [Examples]
/// The following command:
///
///     knapsac get dependency a b
///
/// will return the <OUTPUT_DIRECTORY> of module 'b' which is a dependency of module 'a'
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct Dependency {
    /// Identifier for the dependency
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    ///
    /// [Examples]
    ///     my_a
    ///     "my a"
    ///     my\ a
    #[clap(verbatim_doc_comment)]
    pub(crate) module_identifier: String,
    /// Identifier for the dependency
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    ///
    /// [Examples]
    ///     my_a
    ///     "my a"
    ///     my\ a
    #[clap(verbatim_doc_comment)]
    pub(crate) dependency_identifier: String,
}

impl Dependency {
    pub(crate) fn handle_command(module_identifier: &str, dependency_identifier: &str) {
        let r = Registry::load();
        if let Some(d) = r.get_dependency(module_identifier, dependency_identifier) {
            println!("{}", d.output_directory.display());
        } else {
            exit(1);
        }
    }
}
