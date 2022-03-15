use crate::module::{create_known_module, create_registered_module, Module};
use fmt::Display;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{fmt, fs};

#[derive(Deserialize, Serialize)]
pub(crate) struct Registry {
    modules: Vec<Module>,
}

pub(crate) fn load_registry(registry_path: PathBuf) -> Result<Registry> {
    let data = fs::read_to_string(registry_path).expect("Unable to read file");
    serde_json::from_str(&data)
}

pub(crate) fn save_registry(registry: &Registry, registry_path: &Path) {
    let file = match File::open(registry_path) {
        Ok(f) => f,
        Err(_) => match File::create(registry_path) {
            Ok(f) => f,
            Err(e) => panic!("Could not create file {}: {}", registry_path.display(), e),
        },
    };

    let mut buf_writer = BufWriter::new(file);
    let contents = serde_json::to_string(registry);
    match buf_writer.write_all(contents.unwrap().as_bytes()) {
        Ok(_) => {}
        Err(e) => panic!(
            "Could not write registry to file {}: {}",
            registry_path.display(),
            e
        ),
    }
}

impl Display for Registry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let modules = self
            .modules
            .iter()
            .map(|m| format!("{}", m))
            .collect::<String>();
        write!(f, "{}", modules)
    }
}

pub(crate) fn create_dummy_registry() -> Result<Registry> {
    Ok(Registry {
        modules: vec![
            create_registered_module(
                "~/GitRepositories/JSON/src/JSON.sac",
                "~/GitRepositories/JSON",
                "https://github.com/jcuppen/JSON",
                "~/GitRepositories/JSON/JSON.dep",
            ),
            create_known_module(
                "~/GitRepositories/Stdlib/src/structures/Array.sac",
                "~/GitRepositories/Stdlib",
                "~/GitRepositories/Stdlib/Stdlib.dep",
            ),
        ],
    })
}

impl Registry {
    pub(crate) fn save(&self, registry_path: &PathBuf) {
        let file = match File::open(registry_path) {
            Ok(f) => f,
            Err(_) => match File::create(registry_path) {
                Ok(f) => f,
                Err(e) => panic!("Could not create file {}: {}", registry_path.display(), e),
            },
        };

        let mut buf_writer = BufWriter::new(file);
        let contents = serde_json::to_string(self);
        match buf_writer.write_all(contents.unwrap().as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!(
                "Could not write registry to file {}: {}",
                registry_path.display(),
                e
            ),
        }
    }
}

// impl Registry {
//     pub(crate) fn dump(&self) {
//         self.modules.iter().for_each(|m| m.dump())
//     }

//     pub(crate) fn check_availability(&self, module_id: &str) -> Vec<&Library> {
//         self.libraries
//             .iter()
//             .filter(|lib| lib.modules.contains(&String::from(module_id)))
//             .collect()
//     }
//
//     fn request_clarification(&self, module_id: &String) -> Option<&Library> {
//         let res = self.check_availability(module_id);
//
//         match res[..] {
//             [] => None,
//             [x] => Some(x),
//             _ => choose_candidate(module_id.clone(), res.clone(), false),
//         }
// }

// Choices:
// - if one of my choices resolves a later module [auto-resolve || continue and resolve later]
// - if at the end result in conflicts [retry all || pick retry]
// - how to deal with conflicts with installed libraries?
//
//     pub(crate) fn choose_libraries(&self) -> LibrarySet {
//         let mut requested_libraries = LibrarySet { libraries: vec![] };
//
//         for module_id in options::get_options().modules {
//             let result = self.request_clarification(&module_id);
//             let lib = match result {
//                 None => {
//                     request_retry(self, &module_id);
//                     exit(1)
//                 }
//                 Some(x) => x,
//             };
//
//             requested_libraries.add_library(lib.clone());
//         }
//
//         requested_libraries
//     }
// }
