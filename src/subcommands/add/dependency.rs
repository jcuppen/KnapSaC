
use clap::{Args};
use knapsac_lib2::registry::Registry;


#[derive(Args)]
/// Adds a module dependency to another module
///
/// [Examples]
/// The following command:
///
///     knapsac add dependency a b
///
/// will result add the dependency registered under identifier 'b' to
/// the module registered under identifier 'a".
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
    pub(crate) fn handle_command(
        module_identifier: &str,
        dependency_identifier: &str,
    ) {
        Registry::load().add_dependency(module_identifier, dependency_identifier);
    }
}
