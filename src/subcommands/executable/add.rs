use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;

#[derive(Args)]
/// Adds an executable to the registry
///
/// [Examples]
/// The following command:
///
///     knapsac executable add /home/my_user/a.c
///
/// will result in the following entry being added to the registry:
///
///     {
///         "/home/my_user/a.c": {
///             "dependencies": [...]   
///         }
///     }
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct Add {
    /// Path to the source file representing the executable
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    /// Paths with environment variables are allowed
    /// Path has to point to a file in a git repository
    ///
    /// [Examples]
    ///     /home/my_user/a.c
    ///     /home/my\ user/a.c
    ///     "/home/my user/a.c"
    ///     $HOME/a.c
    #[clap(parse(from_os_str))]
    #[clap(verbatim_doc_comment)]
    pub(crate) source_path: PathBuf,
}

pub(crate) fn handle_command(source_path: PathBuf) {
    Registry::load().add_executable(source_path);
}
