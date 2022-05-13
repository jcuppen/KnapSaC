use std::path::{PathBuf};
use knapsac_lib::module::standalone_module::StandaloneModule;
use knapsac_lib::registry::Registry;
use clap::Args;
use crate::subcommands::Subcommand;

#[derive(Args)]
/// Adds a module to the registry
///
/// [Examples]
/// The following command:
///
///     ./knapsac -r /home/my_user/registry.json add /home/my_user/file.txt
///
/// will result in the following entry being added to the registry:
///
///     {
///         "local_location": "/home/my_user/file.txt"
///     }
///
/// [Errors]
/// An error is returned when:
///     the given <REGISTRY_PATH> does not point to a valid registry file
#[clap(verbatim_doc_comment)]
pub(crate) struct Add {
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
}

impl Subcommand for Add {}

impl Add {
    pub(crate) fn handle_command(registry_path: &PathBuf, module_path: PathBuf) {
        Self::print_load_registry_message(registry_path);
        let mut r = Registry::load(registry_path).unwrap_or_else(|e|Self::print_registry_error(e, registry_path));

        Self::print_message(format!("Creating entry for module located @ {}", module_path.display()));
        let m = match StandaloneModule::create(module_path) {
            Ok(m) => m,
            Err(e) => Self::print_module_error(e),
        };

        Self::print_message("Adding entry to registry");
        r.add_module(m).unwrap_or_else(|e|Self::print_registry_error(e,&registry_path));
        Self::print_message("Successfully added entry to registry");
    }
}
