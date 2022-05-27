use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Args)]
/// Gets a dependency for an executable
///
/// [Examples]
/// The following command:
///
///     knapsac executable get-dependency /home/my_user/a.c b
///
/// will return the <OUTPUT_DIRECTORY> of module 'b'
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct GetDependency {
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

pub(crate) fn handle_command(source_path: &Path, dependency_identifier: &str) {
    let r = Registry::load();
    if let Some(d) = r.get_dependency_for_executable(source_path, dependency_identifier) {
        println!("{}", d.output_path.display());
    } else {
        exit(1);
    }
}
