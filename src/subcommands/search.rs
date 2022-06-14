use clap::ArgEnum;
use clap::Args;
use knapsac_lib::module::Module;
use knapsac_lib::registry::Registry;
use scan_fmt::scanln_fmt;
use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, ArgEnum)]
pub(crate) enum PathType {
    Source,
    Output,
}

#[derive(Args)]
pub(crate) struct Search {
    #[clap(arg_enum)]
    path_type: PathType,
    identifier: String,
}

impl Search {
    fn pick_module(candidates: Vec<(&PathBuf, &Module)>) -> (PathBuf, Module) {
        for (i, (_, v)) in candidates.iter().enumerate() {
            println!("[{}]: {}", i + 1, v.output_path.display());
        }
        println!("Please select desired module:");
        if let Ok(c) = scanln_fmt!("{d}", usize) {
            if c > 0 && c <= candidates.len() {
                let (k, v) = candidates[c - 1];
                return (k.clone(), v.clone());
            }
            return Search::pick_module(candidates);
        }
        Search::pick_module(candidates)
    }

    fn print_path(&self, module: (&PathBuf, &Module)) {
        let path = match &self.path_type {
            PathType::Source => module.0,
            PathType::Output => &module.1.output_path,
        };
        println!("{}", path.display());
    }

    pub(crate) fn handle_command(&self) {
        let r = Registry::load();
        let candidates = r.get_modules(&self.identifier);

        // if candidates.is_empty() {
        // } else if candidates.len() == 1 {
        //     let (&k, &v) = candidates.first().unwrap();
        //     self.print_path((k, v));
        //     //
        // } else {
        //     self.print_path(Self::pick_module(&candidates))
        //     // println!("{}", choice.output_path.display());
        // }

        if candidates.is_empty() {
            println!("ERROR no modules for identifier '{}'", self.identifier);
            exit(1);
        }

        match candidates[..] {
            [] => panic!(),
            [i] => self.print_path(i),
            _ => {
                let (k, v) = &Self::pick_module(candidates);
                self.print_path((k, v))
            }
        }
    }
}
