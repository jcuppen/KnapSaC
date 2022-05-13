use std::path::{Path, PathBuf};
use std::process::exit;
use knapsac_lib::registry::get::FindBy;
use knapsac_lib::registry::Registry;
use clap::Args;
use crate::subcommands::Subcommand;

#[derive(Args)]
/// Removes a module from the registry
///
/// [Examples]
///     ./knapsac -r /home/my_user/registry.json remove /home/my_user/file.txt
///
/// will remove the following entry from the registry:
///
///     {
///         "local_location": "/home/my_user/file.txt"
///     }
///
/// [Errors]
/// An error is returned when:
///     the given <REGISTRY_PATH> does not point to a valid registry file
///
#[clap(verbatim_doc_comment)]
pub(crate) struct Remove {
    /// Path to the module
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    /// Paths with environment variables are allowed
    /// Path has to point to a file in a git repository
    ///
    /// [Examples]
    ///     /home/my_user/file.txt
    ///     /home/my\ user/file.txt
    ///     "/home/my user/file.txt"
    ///     $HOME/file.txt
    #[clap(parse(from_os_str))]
    #[clap(verbatim_doc_comment)]
    pub(crate) module_path: PathBuf,
}

impl Subcommand for Remove {}

impl Remove {
    pub(crate) fn handle_command(registry_path: &PathBuf, module_path: &Path)  {
        Self::print_load_registry_message(registry_path);
        let mut r = Registry::load(registry_path.clone()).unwrap_or_else(|e|Self::print_registry_error(e, registry_path));

        Self::print_search_module_message(module_path);
        let m = r.get_module(FindBy::Location(module_path.to_path_buf())).unwrap_or_else(||{
            Self::print_module_not_found_message(module_path);
            exit(0)
        });

        Self::print_message("Removing entry to registry");
        r.remove_module(&m).unwrap_or_else(|e|Self::print_registry_error(e, registry_path));
        Self::print_message("Successfully removed module from registry")
    }
}
