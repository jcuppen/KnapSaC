use clap::ArgEnum;
use clap::Args;
use knapsac_lib::module::Module;
use knapsac_lib::registry::Registry;
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
    fn print_paths(&self, candidates: Vec<(&PathBuf, &Module)>, package_candidates: Vec<(&PathBuf, &Module)>) {
        println!("[0]: Try to use fallback mechanism");
        for (i, (_, v)) in candidates.iter().enumerate() {
            println!("[{}]: {}", i + 1, v.output_path.display());
        }
        for (i, (_, v)) in package_candidates.iter().enumerate() {
            println!("[{}]: {}", i + 1 + candidates.len(), v.output_path.display());
        }
        print!("Which {} do you want to use? [0..{}]", self.identifier, candidates.len() + package_candidates.len());
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
        let mut candidates = r.search_modules_by_id(&self.identifier);
        let mut p_candidates = r.search_package_modules(&self.identifier);

        println!("{:?}", candidates);
        println!("{:?}", p_candidates);

        candidates.sort_by(|(ap, _), (bp, _)| ap.partial_cmp(bp).unwrap());
        p_candidates.sort_by(|(ap,_), (bp,_)| ap.partial_cmp(bp).unwrap());

        if candidates.is_empty() && p_candidates.is_empty() {
            eprintln!("No modules for identifier '{}'", self.identifier);
            exit(1);
        }

        let mut all = vec![];
        all.append(&mut candidates);
        all.append(&mut p_candidates);

        let candidates_len = candidates.len();
        let total = candidates_len + p_candidates.len();

        match self.choice {
            None => match all[..] {
                [] => panic!(),
                [i] => self.print_path(i),
                _ => {
                    self.print_paths(candidates, p_candidates);
                }
            },
            Some(c) if (1..candidates_len).contains(&c) => {
                self.print_path(candidates[c - 1]);
            }
            Some(c) if ((candidates_len + 1)..(total + 1)).contains(&c) => {
                self.print_path(p_candidates[c - (1 + candidates_len)])
            },
            Some(_) => panic!("Invalid choice"),
        }
    }
}
