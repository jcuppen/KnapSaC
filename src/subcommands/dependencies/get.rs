use clap::Args;
use knapsac_lib::dependency::Dependency;
use knapsac_lib::registry::Registry;
use std::path::Path;
use std::process::exit;

#[derive(Args)]
pub(crate) struct Get {
    dependency_identifier: String,
}

impl Get {
    fn print_path(&self, p: &Path) {
        println!("{}", p.display());
    }

    pub(crate) fn handle_command(&self, depender: &Path) {
        let r = Registry::load();
        let opt = r.get_dependency(depender, &self.dependency_identifier);

        if let Some(d) = opt {
            match d {
                Dependency::Stray(_, o) => self.print_path(o),
                Dependency::Standalone(_) => {
                    self.print_path(&r.dep_to_module(d).unwrap().output_path)
                }
                Dependency::Package => panic!(),
            }
        } else {
            println!("ERROR dep get");
            exit(1);
        }
    }
}
