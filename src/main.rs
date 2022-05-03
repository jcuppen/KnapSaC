extern crate core;

use crate::options::{Cli, Command};

mod options;

use clap::Parser;
use knapsac_lib::{download, initialize_registry, load_registry};
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();

    let registry_path: PathBuf = cli.registry_path;

    let mut registry = initialize_registry();

    match cli.command {
        Command::Add {package_location}=> {
            registry = load_registry(&registry_path);
            registry.add(&package_location);
        }
        Command::Remove {package_location} => {
            registry = load_registry(&registry_path);
            registry.remove(&package_location);
        }
        Command::Initialize => {}
        Command::Download {package_location, target_location} => {
            registry = load_registry(&registry_path);
            download(&mut registry, package_location, &target_location);
        }
        // Command::AddDependency { path, value } => registry.add_dependency(&path, value),
        // Command::RemoveDependency { path, value } => registry.remove_dependency(&path, value),
    }
    registry.save(&registry_path);
}
