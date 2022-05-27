use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;

#[derive(Args)]
/// Adds a module to the registry
///
/// [Examples]
/// The following command:
///
///     knapsac module add a /home/my_user/a/out
///
/// will result in the following entry being added to the registry:
///
///     {
///         "a" : {
///             "output_path": "/home/my_user/a/out",
///             "dependencies": [],    
///         }
///     }
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct Add {
    /// Identifier for the dependency
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    ///
    /// [Examples]
    ///     my_a
    ///     "my a"
    ///     my\ a
    #[clap(verbatim_doc_comment)]
    pub(crate) identifier: String,
    /// Path to the output directory where the module leaves its build artifacts (e.g. libraries)
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    /// Paths with environment variables are allowed
    /// Path has to point to a file in a git repository
    ///
    /// [Examples]
    ///     /home/my_user/a/out
    ///     /home/my\ user/a/out
    ///     "/home/my user/a/out"
    ///     $HOME/a/out
    #[clap(parse(from_os_str))]
    #[clap(verbatim_doc_comment)]
    pub(crate) output_path: PathBuf,
}

pub(crate) fn handle_command(identifier: String, output_path: PathBuf) {
    Registry::load().add_module(identifier, output_path);
}
