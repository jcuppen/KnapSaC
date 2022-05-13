use std::path::{Path, PathBuf};
use std::process::exit;
use clap::Args;
use knapsac_lib::registry::get::FindBy;
use knapsac_lib::registry::Registry;
use crate::subcommands::Subcommand;

#[derive(Args)]
/// Adds a module dependency to another module's manifest
///
/// [Manifest File]
/// The manifest file is a file that maintains meta data about a module
/// Given a module `a.txt` the associated manifest file would be `a.knapsac`.
///
/// [Examples]
/// The following command:
///
///     ./knapsac -r /home/my_user/registry.json add-dependency /home/my_user/a.txt /home/my_user/b.txt
///
/// will result in the following entry being added to the modules dependency file:
///
///     {
///         "local_location": "/home/my_user/file.txt"
///     }
///
/// If the dependencies file does not exist it will be created.
///
///
/// [Errors]
/// An error is returned when:
///     the given <REGISTRY_PATH> does not point to a valid registry file
#[clap(verbatim_doc_comment)]
pub(crate) struct AddDependency {
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
    ///
    /// [Errors]
    /// An error is returned when:
    ///     - the given <MODULE_PATH> does point to an existing module
    #[clap(parse(from_os_str))]
    #[clap(verbatim_doc_comment)]
    pub(crate) module_path: PathBuf,
    /// Path to the dependency
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
    ///
    /// [Errors]
    /// An error is returned when:
    ///     - the given <MODULE_PATH> does point to an existing module
    ///     - the given <MODULE_PATH> is not in the registry
    #[clap(parse(from_os_str))]
    #[clap(verbatim_doc_comment)]
    pub(crate) dependency_path: PathBuf,
}

impl Subcommand for AddDependency {}

impl AddDependency {
    pub(crate) fn handle_command(registry_path: &PathBuf, module_path: &Path, dependency_path: &Path) {
        Self::print_load_registry_message(registry_path);
        let r = Registry::load(registry_path).unwrap_or_else(|e|Self::print_registry_error(e,registry_path));

        Self::print_search_module_message(module_path);
        let m = r.get_module(FindBy::Location(module_path.to_path_buf())).unwrap_or_else(||{
            Self::print_module_not_found_message(module_path);
            exit(0)
        });

        Self::print_search_module_message(dependency_path);
        let d = r.get_module(FindBy::Location(dependency_path.to_path_buf())).unwrap_or_else(||{
            Self::print_module_not_found_message(dependency_path);
            Self::print_message("Try adding the module first");
            exit(0)
        });

        if let Err(e) = m.add_module_dependency(d){
            Self::print_module_error(e);
        }
        Self::print_message(format!("Successfully added dependency {} to module {}", dependency_path.display(), module_path.display()));
    }
}
