use clap::Args;
use knapsac_lib::registry::Registry;
use std::process::exit;
use knapsac_lib::entry::Entry;

#[derive(Args)]
pub(crate) struct Get {
    dependency_identifier: String,
}

impl Get {
    pub(crate) fn handle_command(&self, depender: Entry) {
        let r = Registry::load();
        let opt = r.get_dependency(depender, &self.dependency_identifier);

        if let Some(d) = opt {
            println!("{}", d.output_path.display());
        } else {
            println!("ERROR dep get");
            exit(1);
        }
    }
}
