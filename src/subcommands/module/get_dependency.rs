use clap::Args;
use knapsac_lib::registry::Registry;
use std::process::exit;

#[derive(Args)]
/// Gets a dependency for a module
///
/// [Examples]
/// The following command:
///
///     knapsac module get-dependency a b
///
/// will return the <OUTPUT_PATH> of module 'b'
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct GetDependency {
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

pub(crate) fn handle_command(module_identifier: &str, dependency_identifier: &str) {
    let r = Registry::load();
    if let Some(d) = r.get_dependency_for_module(module_identifier, dependency_identifier) {
        println!("{}", d.output_path.display());
    } else {
        exit(1);
    }
}
