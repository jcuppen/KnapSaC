use clap::Args;
use knapsac_lib::registry::Registry;

#[derive(Args)]
/// Removes a module from the registry
///
/// [Examples]
///     knapsac module remove a
///
/// will remove the following entry from the registry:
///
///     {
///         "a" : {
///             "output_path": "...",
///             "dependencies": [...],    
///         }
///     }
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct Remove {
    /// Identifier for the module
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    ///
    /// [Examples]
    ///     my_a
    ///     "my a"
    ///     my\ a
    #[clap(verbatim_doc_comment)]
    pub(crate) identifier: String,
}

pub(crate) fn handle_command(identifier: &str) {
    Registry::load().remove_module(identifier);
}
