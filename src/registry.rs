use crate::library::{create_library, Library, LibrarySet};
use crate::{choose_candidate, options};
use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct Registry {
    libraries: Vec<Library>,
}

// pub(crate) fn create_registry(libraries: Vec<Library>) -> Registry {
//     Registry { libraries }
// }

pub(crate) fn create_dummy_registry() -> Registry {
    Registry {
        libraries: vec![
            create_library("StdLib", "https://github.com/SaCBase/StdLib", vec!["Array"]),
            create_library("SDL", "https://gitlab.com/SDLDevs/SDL-for-SaC", vec!["SDL"]),
            create_library("SDL", "https://github.com/SaCBase/SDL", vec!["SDL"]),
            create_library("JSON", "https://github.com/jcuppen/JSON", vec!["JSON"]),
            create_library(
                "Images",
                "https://sac-images.com/repository",
                vec!["png", "jpg", "svg"],
            ),
            create_library(
                "Fonts",
                "https://sac-fonts.com/repository",
                vec!["otf", "ttf", "svg"],
            ),
        ],
    }
}

impl Registry {
    pub(crate) fn check_availability(&self, module_id: &str) -> Vec<&Library> {
        self.libraries
            .iter()
            .filter(|lib| lib.modules.contains(&String::from(module_id)))
            .collect()
    }

    fn request_clarification(&self, module_id: String) -> Option<&Library> {
        let res = self.check_availability(&*module_id);

        match res[..] {
            [] => None,
            [x] => Some(x),
            _ => choose_candidate(module_id, res.clone(), false),
        }
    }

    pub(crate) fn resolve_ambiguities(&self) -> LibrarySet {
        let mut requested_libraries: HashSet<Library> = HashSet::new();

        for module_id in options::get_options().modules {
            let result = self.request_clarification(module_id);
            match result {
                None => {}
                Some(lib) => {
                    requested_libraries.insert(lib.clone());
                }
            }
        }

        LibrarySet {
            libraries: requested_libraries,
        }
    }
}
