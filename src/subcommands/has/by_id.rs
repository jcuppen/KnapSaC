use clap::Args;
use knapsac_lib::registry::Registry;
use std::process::exit;

#[derive(Args)]
pub(crate) struct ById {
    identifier: String,
}

impl ById {
    pub(crate) fn handle_command(&self) {
        let r = Registry::load();

        if !r.has_module_id(&self.identifier) {
            exit(1);
        }
    }
}
