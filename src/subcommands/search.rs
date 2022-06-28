use clap::ArgEnum;
use clap::Args;
use knapsac_lib::module::Module;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, ArgEnum)]
enum PathType {
    Source,
    Output,
}

#[derive(Args)]
pub(crate) struct Search {
    #[clap(arg_enum)]
    path_type: PathType,
    identifier: String,
    // #[clap(short)]
    // link_paths: Vec<PathBuf>,
    choice: Option<usize>,
}

impl Search {
    fn print_paths(&self, candidates: Vec<(&PathBuf, &Module)>) {
        println!("[0]: Try to use fallback mechanism");
        for (i, (_, v)) in candidates.iter().enumerate() {
            println!("[{}]: {}", i + 1, v.output_path.display());
        }
        println!(
            "Which {} do you want to use? [0..{}]",
            self.identifier,
            candidates.len()
        );
    }

    fn print_path(&self, module: (&PathBuf, &Module)) {
        let path = match &self.path_type {
            PathType::Source => module.0,
            PathType::Output => &module.1.output_path,
        };
        println!("{}", path.display());
    }

    pub(crate) fn handle_command(&mut self) {
        let r = Registry::load();
        let mut candidates = r.search_modules_by_id(&self.identifier);
        let mut p_candidates = r.search_package_modules(&self.identifier);

        candidates.sort_by(|(ap, _), (bp, _)| ap.partial_cmp(bp).unwrap());
        p_candidates.sort_by(|(ap, _), (bp, _)| ap.partial_cmp(bp).unwrap());

        let mut all = vec![];
        all.append(&mut candidates);
        all.append(&mut p_candidates);

        // let canonical_link_paths: Vec<PathBuf> = self
        //     .link_paths
        //     .iter_mut()
        //     .map(|p| p.canonicalize().unwrap())
        //     .collect();

        // let total_matches = all
        //     .iter()
        //     .filter(|(_, m)| canonical_link_paths.contains(&m.output_path))
        //     .count();

        // match total_matches {
        //     0 => {}
        //     1 => {
        //         all.retain(|(_, m)| self.link_paths.contains(&m.output_path));
        //     }
        //     _ => {
        //         eprintln!("Multiple modules share output path with paths that will be passed to the linker.");
        //         eprintln!(
        //             "This makes it very unpredictable to guarantee intended usage of modules."
        //         );
        //         exit(1);
        //     }
        // }

        if all.is_empty() {
            eprintln!("No modules for identifier '{}'", self.identifier);
            exit(1);
        }

        match self.choice {
            None => match all[..] {
                [] => panic!(),
                [i] => self.print_path(i),
                _ => self.print_paths(all),
            },
            Some(c) if (1..all.len() + 1).contains(&c) => {
                self.print_path(all[c - 1]);
            }
            Some(_) => panic!("Invalid choice"),
        }
    }
}
