use crate::library::{create_library, Library, LibrarySet};
use crate::{choose_candidate, options, request_retry};
use std::process::exit;

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
            create_library(
                "SDL_A",
                "https://gitlab.com/SDLDevs/SDL-for-SaC",
                vec!["SDL"],
            ),
            create_library("SDL_B", "https://github.com/SaCBase/SDL", vec!["SDL"]),
            create_library("JSON", "https://github.com/jcuppen/JSON", vec!["JSON"]),
            create_library(
                "Images",
                "https://sac-images.com/repository",
                vec!["png", "jpg", "svg", "abc"],
            ),
            create_library(
                "Fonts",
                "https://sac-fonts.com/repository",
                vec!["otf", "ttf", "svg", "abc"],
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

    fn request_clarification(&self, module_id: &String) -> Option<&Library> {
        let res = self.check_availability(module_id);

        match res[..] {
            [] => None,
            [x] => Some(x),
            _ => choose_candidate(module_id.clone(), res.clone(), false),
        }
    }

    // Choices:
    // - if one of my choices resolves a later module [auto-resolve || continue and resolve later]
    // - if at the end result in conflicts [retry all || pick retry]
    // - how to deal with conflicts with installed libraries?

    pub(crate) fn choose_libraries(&self) -> LibrarySet {
        let mut requested_libraries = LibrarySet { libraries: vec![] };

        for module_id in options::get_options().modules {
            let result = self.request_clarification(&module_id);
            let lib = match result {
                None => {
                    request_retry(self, &module_id);
                    exit(1)
                }
                Some(x) => x,
            };

            requested_libraries.add_library(lib.clone());
        }

        requested_libraries
    }
}
