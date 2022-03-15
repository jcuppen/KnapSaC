use crate::module::{create_known_module, create_registered_module, Module};
use serde::Deserialize;
use serde_json::Result;
use std::fs;
use std::path::PathBuf;
// use crate::module::ModuleSet;
// use crate::{choose_candidate, options, request_retry};
// use std::process::exit;

#[derive(Deserialize)]
pub(crate) struct Registry {
    modules: Vec<Module>,
}

pub(crate) fn load_registry(registry_path: PathBuf) -> Result<Registry> {
    let data = fs::read_to_string(registry_path).expect("Unable to read file");
    serde_json::from_str(&data)
}

// pub(crate) fn create_dummy_registry() -> Registry {
//     Registry {
//         modules: vec![
//             create_registered_module(
//                 "/Users/job/GitRepositories/JSON/src/JSON.sac",
//                 "/Users/job/GitRepositories/JSON",
//                 "https://github.com/jcuppen/JSON",
//                 "/Users/job/GitRepositories/JSON/JSON.dep",
//             ),
//             create_known_module(
//                 "/Users/job/GitRepositories/Stdlib/src/structures/Array.sac",
//                 "/Users/job/GitRepositories/Stdlib/Stdlib.dep",
//             ),
//         ],
//     }
// }

impl Registry {
    pub(crate) fn dump(&self) {
        self.modules.iter().for_each(|m| m.dump())
    }

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
}

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
