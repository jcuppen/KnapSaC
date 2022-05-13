use std::path::Path;
use knapsac_lib::registry::Registry;
use clap::{Args};
use crate::subcommands::Subcommand;

#[derive(Args)]
/// Creates an empty registry at the given <REGISTRY_PATH>
///
/// Overwrites any file already present
#[clap(verbatim_doc_comment)]
pub(crate) struct Initialize;

impl Subcommand for Initialize {}

impl Initialize {
    pub(crate) fn handle_command<P: AsRef<Path>>(registry_path: P) {
        let r = registry_path.as_ref();
        Self::print_message(format!("Initializing new registry file @ {}", r.display()));
        if r.exists() {
            Self::print_warning("Overwriting existing file!");
        }
        if let Err(e) = Registry::initialize(r) {
            Self::print_registry_error(e, &registry_path);
        }
        Self::print_message(format!("Successfully initialized new registry file @ {}", r.display()));
    }
}
