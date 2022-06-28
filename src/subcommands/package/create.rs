use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;

#[derive(Args)]
pub(crate) struct Create {
    package_root: PathBuf,
    compiler_command_name: String,
    output_option: String,
}

impl Create {
    pub(crate) fn handle_command(&self, identifier: &str) {
        let mut r = Registry::load();
        r.package(
            identifier,
            &self.package_root.canonicalize().unwrap(),
            self.compiler_command_name.clone(),
            self.output_option.clone(),
        );

        println!(
            "Please: \
            \tclean up any remaining build artifacts at old locations.\
            \trecompile all modules that depend on modules belonging to this package."
        );
    }
}
