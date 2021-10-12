mod input;
mod library;
mod options;
mod registry;

#[macro_use]
extern crate prettytable;

use crate::input::boolean::BooleanInput;
use crate::input::natural_number::NaturalNumberInput;
use crate::library::{Library, LibrarySet};
use crate::registry::create_dummy_registry;
use prettytable::{format, Table};
use std::io;
use std::io::Write;
use std::process::exit;

fn choose_candidate(
    module_id: String,
    libraries: Vec<&Library>,
    is_retry: bool,
) -> Option<&Library> {
    if !is_retry {
        println!("Multiple libraries found that provide `{}`", module_id);
    }

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    libraries.iter().enumerate().for_each(|(i, lib)| {
        table.add_row(row![
            format!("[{}]", i + 1),
            lib.id,
            format!("@ {}", lib.location)
        ]);
    });

    table.printstd();

    print!("Please choose an option: ");
    io::stdout().flush().unwrap();

    match input::natural_number::parse(1, libraries.len()) {
        NaturalNumberInput::Number(i) => Some(libraries[i - 1]),
        NaturalNumberInput::TooLow(i) | NaturalNumberInput::TooHigh(i) => {
            println!("`{}` is not a valid option. Please choose again:", i);
            choose_candidate(module_id, libraries, true)
        }
        NaturalNumberInput::NaN(str) => {
            println!("`{}` is not a valid option. Please choose again:", str);
            choose_candidate(module_id, libraries, true)
        }
    }
}

fn request_conformation(library_set: &LibrarySet) -> BooleanInput {
    if library_set.empty() {
        println!("No modules were requested!");
        exit(0)
    }

    println!("The following libraries will be installed:");

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    library_set.libraries.iter().for_each(|lib| {
        table.add_row(row![lib.id, format!("@ {}", lib.location)]);
    });

    table.printstd();

    print!("Is this correct? ([y]es/[n]o): ");
    io::stdout().flush().unwrap();

    input::boolean::parse()
}

fn process_response(library_set: &LibrarySet) {
    let response = request_conformation(library_set);

    if !response.is_valid() {
        println!("Invalid {}", response.invalid_reason());
    }

    if response.is_affirmative() {
        library_set.install()
    } else {
        println!("No packages will be installed!")
    }
}

fn main() {
    let dummy_registry = create_dummy_registry();
    let library_set = dummy_registry.resolve_ambiguities();

    process_response(&library_set);
}
