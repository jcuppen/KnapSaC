extern crate core;

use crate::options::{Cli, Command};

mod options;

use clap::Parser;
use knapsac_lib::dependency::Dependency;
use knapsac_lib::module::Module;
use knapsac_lib::package::Package;
use knapsac_lib::registry::Registry;

fn main() {
    let cli = Cli::parse();

    let registry_path = cli.registry_path;

    match cli.command {
        Command::Add { package_path } => {
            let mut registry = Registry::load(&registry_path);
            let package = Package::create(package_path);
            registry.add(package);
        }
        Command::Remove { package_path } => {
            let registry = Registry::load(&registry_path);
            let found_package = registry.get_by_local_location(&package_path);

            match found_package {
                None => println!("No package found in registry for path '{}'", package_path.display()),
                Some(package) => {
                    let mut registry = Registry::load(&registry_path);
                    registry.remove(package);
                }
            }
        }
        Command::Initialize => {
            Registry::initialize(&registry_path);
        },
        Command::Download {
            package_location,
            package_root,
        } => {
            let mut registry = Registry::load(&registry_path);
            let package = Package::download(package_location, &package_root);
            registry.add(package);
        }
        Command::AddDependency {
            value,
            package_path,
        } => {
            let registry = Registry::load(&registry_path);
            let found_package = registry.get_by_local_location(&package_path);
            match found_package {
                None => println!("No package found in registry for path '{}'", package_path.display()),
                Some(package) => {
                    let dependency = Dependency::create(value);
                    package.add_dependency(dependency)
                },
            }
        }
        Command::RemoveDependency {
            value,
            package_path,
        } => {
            let registry = Registry::load(&registry_path);
            let found_package = registry.get_by_local_location(&package_path);
            match found_package {
                None => println!("No package found in registry for path '{}'", package_path.display()),
                Some(package) => {
                    let dependency = Dependency::create(value);
                    package.remove_dependency(&dependency)
                },
            }
        },
        Command::AddModule { module_path, identifier} => {
            let registry = Registry::load(&registry_path);
            let found_package = registry.get_by_local_location(&module_path);
            match found_package {
                None => println!("No package found in registry for path '{}'", module_path.display()),
                Some(package) => {
                    let relative_module_path = package.strip_prefix(module_path);
                    let module = Module::create(relative_module_path, identifier);
                    package.add_module(module);
                }
            }
        },
        Command::RemoveModule { module_path} => {
            let registry = Registry::load(&registry_path);
            let found_package = registry.get_by_local_location(&module_path);
            match found_package {
                None => println!("No package found in registry for path '{}'", module_path.display()),
                Some(package) => {
                    let relative_module_path = package.strip_prefix(&module_path);
                    match package.get_module_by_location(relative_module_path) {
                        Some(module) => {
                            package.remove_module(&module);
                        }
                        None => println!("No module found with module path '{}'", module_path.display())
                    }
                }
            }
        },
        Command::Search { modules} => {
            let registry = Registry::load(&registry_path);
            let i = registry.search_by_module_identifiers(&modules);
            for p in i {
                println!("{}", p)
            }
        }
    }
}
