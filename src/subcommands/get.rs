use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;
use std::process::exit;

#[derive(Args)]
pub(crate) struct Get {
    source_file: PathBuf,
}

impl Get {
    pub(crate) fn handle_command(&self) {
        let r = Registry::load();
        let module = r.get_module(&self.source_file.canonicalize().unwrap());

        if let Some(m) = module {
            println!("{}", m.output_path.display());
        } else {
            println!("ERROR get module");
            exit(1);
        }
    }
}
