extern crate core;

use crate::options::{Cli, Command};

mod options;

use clap::Parser;
use knapsac_lib::registry::Registry;

fn main() {
    let cli = Cli::parse();

    let registry_path = cli.registry_path;

    match cli.command {
        Command::Add {package_location}=> {
            let mut registry = Registry::load(&registry_path);
            registry.add(&package_location);
            registry.save(&registry_path);
        }
        Command::Remove {package_location} => {
            let mut registry = Registry::load(&registry_path);
            registry.remove(&package_location);
            registry.save(&registry_path);
        }
        Command::Initialize => {
            let registry = Registry::initialize();
            registry.save(&registry_path);
        }
        Command::Download {package_location, target_location} => {
            let mut registry = Registry::load(&registry_path);
            registry.download(package_location, &target_location);
            registry.save(&registry_path);
        }
        Command::AddDependency { package_location, value } => {
            let registry = Registry::load(&registry_path);
            registry.add_dependency(&package_location, value)
        },
        Command::RemoveDependency { package_location, value } => {
            let registry = Registry::load(&registry_path);
            registry.remove_dependency(&package_location, value)
        },
    }
}
