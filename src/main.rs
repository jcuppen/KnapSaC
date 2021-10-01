mod input;
mod options;
mod package;
mod registry;

use crate::input::boolean::BooleanInput;
use crate::input::natural_number::NaturalNumberInput;

use crate::package::{create_package, Package};

use crate::registry::{create_registry, Registry};

use std::io;
use std::io::Write;

use std::process::exit;

fn choose_candidate(packages: Vec<&Package>, is_retry: bool) -> Option<&Package> {
    if !is_retry {
        println!(
            "Multiple candidates found for package `{}`!",
            packages.first().unwrap().id
        );
    }

    for (i, p) in packages.iter().enumerate() {
        println!("  - [{}] {}", i + 1, p.location);
    }

    print!("Please choose an option: ");
    io::stdout().flush().unwrap();

    match input::natural_number::parse(1, packages.len()) {
        NaturalNumberInput::Number(i) => Some(packages[i - 1]),
        NaturalNumberInput::TooLow(i) | NaturalNumberInput::TooHigh(i) => {
            println!("`{}` is not a valid option. Please choose again:", i);
            choose_candidate(packages, true)
        }
        NaturalNumberInput::NaN(str) => {
            println!("`{}` is not a valid option. Please choose again:", str);
            choose_candidate(packages, true)
        }
    }
}

fn clarify(reg: &Registry, pkg_id: String) -> Option<&Package> {
    let res = reg.check_availability(pkg_id);

    match res[..] {
        [] => None,
        [x] => Some(x),
        _ => choose_candidate(res.clone(), false),
    }
}

fn request_conformation(packages: &Vec<Package>) -> BooleanInput {
    let formatted_packages = packages
        .iter()
        .map(|pkg| format!("  - {} @ {}", pkg.id, pkg.location))
        .collect::<Vec<String>>()
        .join("\n");

    println!("You have requested the following package(s) to be installed:");
    println!("{}", formatted_packages);
    print!("Is this correct? ([y]es/[n]o): ");
    io::stdout().flush().unwrap();

    input::boolean::parse()
}

fn process_response(packages: &Vec<Package>, response: BooleanInput) {
    if !response.is_valid() {
        println!("Invalid {}", response.invalid_reason());
        request_conformation(packages);
    }

    if response.is_affirmative() {
        println!("The requested packages will be installed!")
    } else {
        println!("No packages will be installed!")
    }
    exit(0);
}

fn main() {
    let dummy_registry: Registry = create_registry(vec![
        create_package("A", "A-location"),
        create_package("B", "B-location-1"),
        create_package("B", "B-location-2"),
        create_package("C", "C-location"),
    ]);
    let opt = options::get_options();
    let package_identifiers = opt.packages;

    if package_identifiers.len() == 0 {
        println!("No packages were requested!");
        exit(0)
    }

    let mut requested_packages = vec![];

    for pkg_id in package_identifiers {
        let r = clarify(&dummy_registry, pkg_id);
        match r {
            None => {}
            Some(pkg) => requested_packages.push(pkg.clone()),
        }
    }

    let answer = request_conformation(&requested_packages);

    process_response(&requested_packages, answer);
}
