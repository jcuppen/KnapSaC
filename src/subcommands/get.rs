
use clap::Args;
use knapsac_lib::registry::Registry;
use std::process::exit;
use knapsac_lib::entry::Entry;

#[derive(Args)]
pub(crate) struct Get {}

impl Get {
    pub(crate) fn handle_command(&self, entry: &Entry) {
        let r = Registry::load();
        let path = match entry {
            Entry::Executable(source_path) => {
                r.get_executable(source_path).map(|_| source_path)
            },
            Entry::StandaloneModule(identifier) => {
                r.get_module(identifier).map(|m| &m.output_path)
            }
            Entry::PackageModule(_,_) => {
                panic!()
            }
        };

        if let Some(p) = path {
            println!("{}", p.display());
        } else {
            match entry {
                Entry::Executable(s) => {
                    println!("ERROR get {}", s.display());
                },
                Entry::StandaloneModule(i) => {
                    println!("ERROR get {}", i);
                },
                Entry::PackageModule(_,_) => {panic!()}
            }
            exit(1);
        }
    }
}
