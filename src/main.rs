mod input;
mod module;
mod options;
mod registry;

// #[macro_use]
// extern crate prettytable;

use crate::registry::load_registry;

use git2::Repository;
use std::process::exit;

// fn choose_candidate(
//     module_id: String,
//     libraries: Vec<&Library>,
//     is_retry: bool,
// ) -> Option<&Library> {
//     if !is_retry {
//         println!("Multiple libraries found that provide `{}`", module_id);
//     }
//
//     let mut table = Table::new();
//     table.set_format(*format::consts::FORMAT_CLEAN);
//
//     libraries.iter().enumerate().for_each(|(i, lib)| {
//         table.add_row(row![
//             format!("[{}]", i + 1),
//             lib.id,
//             format!("@ {}", lib.location)
//         ]);
//     });
//
//     table.printstd();
//
//     print!("Please choose an option: ");
//     io::stdout().flush().unwrap();
//
//     match input::natural_number::parse(1, libraries.len()) {
//         NaturalNumberInput::Number(i) => Some(libraries[i - 1]),
//         NaturalNumberInput::TooLow(i) | NaturalNumberInput::TooHigh(i) => {
//             println!("`{}` is not a valid option. Please choose again:", i);
//             choose_candidate(module_id, libraries, true)
//         }
//         NaturalNumberInput::NaN(str) => {
//             println!("`{}` is not a valid option. Please choose again:", str);
//             choose_candidate(module_id, libraries, true)
//         }
//     }
// }
//
// fn request_conformation_install(library_set: &LibrarySet) -> BooleanInput {
//     if library_set.empty() {
//         println!("No modules were requested!");
//         exit(0)
//     }
//
//     println!("The following libraries will be installed:");
//
//     let mut table = Table::new();
//     table.set_format(*format::consts::FORMAT_CLEAN);
//
//     library_set.libraries.iter().for_each(|lib| {
//         table.add_row(row![lib.id, format!("@ {}", lib.location)]);
//     });
//
//     table.printstd();
//
//     print!("Is this correct? ([y]es/[n]o): ");
//     io::stdout().flush().unwrap();
//
//     input::boolean::parse()
// }

// fn request_conformation_retry() -> BooleanInput {
//     print!("Do you want to try a different set of libraries? ([y]es/[n]o): ");
//     io::stdout().flush().unwrap();
//
//     input::boolean::parse()
// }

// fn process_response_installation(library_set: &LibrarySet) {
//     let response = request_conformation_install(library_set);
//
//     if !response.is_valid() {
//         println!("Invalid {}", response.invalid_reason());
//     }
//
//     if response.is_affirmative() {
//         library_set.install()
//     } else {
//         println!("No packages will be installed!")
//     }
// }

// fn request_retry(registry: &Registry, conflicting_module_id: &String) {
//     println!(
//         "With the requested set of libraries it is impossible to resolve `{}`",
//         conflicting_module_id
//     );
//
//     let response = request_conformation_retry();
//
//     if !response.is_valid() {
//         println!("Invalid {}", response.invalid_reason());
//     }
//
//     if response.is_affirmative() {
//         registry.choose_libraries();
//     } else {
//         println!("No packages will be installed!")
//     }
// }

fn main() {
    let registry_path = options::get_options().registry_filepath;
    let new_repo_path = options::get_options().add;

    let repo = match Repository::open(&new_repo_path) {
        Ok(r) => r,
        Err(_) => {
            println!("Failed to parse '{}'", new_repo_path.display());
            exit(1)
        }
    };

    // println!("{:?}", repo.remotes().unwrap().);
    // repo.remotes()
    //     .iter()
    //     .for_each(|s| s.iter().for_each(|a| println!("{}", a.unwrap())));
    for stringArray in repo.remotes() {
        for opt_str in stringArray.iter() {
            match opt_str {
                None => {}
                Some(s) => {
                    let remote = repo.find_remote(s).unwrap();
                    println!("remote: {} url: {}", s, remote.url().unwrap())
                }
            }
        }
    }

    let registry = match load_registry(&registry_path) {
        Ok(r) => r,
        Err(_) => {
            println!("Failed to parse '{}'", registry_path.display());
            exit(1)
        }
    };

    if options::get_options().dump_registry {
        registry.dump();
    }

    registry.save(&registry_path);
}
