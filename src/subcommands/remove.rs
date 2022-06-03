use clap::Args;
use knapsac_lib::entry::Entry;
use knapsac_lib::registry::Registry;

#[derive(Args)]
pub(crate) struct Remove {}

impl Remove {
    pub(crate) fn handle_command(&self, entry: Entry) {
        let mut r = Registry::load();
        r.remove(entry);
    }
}
