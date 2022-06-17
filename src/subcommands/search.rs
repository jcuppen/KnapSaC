// use std::io::stdin;
use clap::ArgEnum;
use clap::Args;
use knapsac_lib::module::Module;
use knapsac_lib::registry::Registry;
// use scan_fmt::scanln_fmt;
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
    choice: Option<usize>,
}

impl Search {
    fn print_paths(&self, candidates: Vec<(&PathBuf, &Module)>) {
        println!("Which {} do you want to use?", self.identifier);
        println!("\t[0]: Try to use fallback mechanism");
        for (i, (_, v)) in candidates.iter().enumerate() {
            println!("\t[{}]: {}", i + 1, v.output_path.display());
        }
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
        let mut candidates = r.get_modules(&self.identifier);
        candidates.sort_by(|(ap, _), (bp, _)| ap.partial_cmp(bp).unwrap());

        if candidates.is_empty() {
            eprintln!("No modules for identifier '{}'", self.identifier);
            exit(1);
        }

        match self.choice {
            None => match candidates[..] {
                [] => panic!(),
                [i] => self.print_path(i),
                _ => {
                    self.print_paths(candidates);
                }
            },
            Some(c) => match candidates[..] {
                [] => panic!(),
                [i] => self.print_path(i),
                _ => {
                    if c == 0 && c - 1 > candidates.len() {
                        eprintln!("Invalid choice");
                    }
                    self.print_path(candidates[c - 1]);
                }
            },
        }
    }
}
