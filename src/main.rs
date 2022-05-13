extern crate core;

mod new_options;
mod subcommands;

use crate::new_options::{Cli, Command};
use clap::Parser;
use std::path::PathBuf;
use crate::subcommands::add::Add;
use crate::subcommands::add_dependency::AddDependency;
use crate::subcommands::initialize::Initialize;
use crate::subcommands::remove::Remove;

fn main() {
    let cli = Cli::parse();

    let registry_path: PathBuf = cli.registry_path;

    match cli.command {
        Command::Initialize(_) => Initialize::handle_command(registry_path),
        Command::Add(Add { module_path }) => {
            Add::handle_command(&registry_path, module_path)
        },
        Command::Remove(Remove { module_path }) => {
            Remove::handle_command(&registry_path, &module_path)
        }
        Command::AddDependency(AddDependency { module_path, dependency_path}) => {
            AddDependency::handle_command(&registry_path, &module_path, &dependency_path)
        }
    }

    // match cli.command {
    //     Command::Add { package_path } => match Registry::load(&registry_path) {
    //         Ok(mut registry) => match Package::create(package_path) {
    //             Ok(package) => registry
    //                 .add_package(package)
    //                 .unwrap_or_else(|e| print_registry_error(e)),
    //             Err(e) => print_package_error(e),
    //         },
    //         Err(e) => print_registry_error(e),
    //     },
    //     Command::Remove { package_path } => match Registry::load(&registry_path) {
    //         Ok(mut registry) => match registry.get_package(FindBy::LocalLocation(package_path)) {
    //             Ok(Some(package)) => registry
    //                 .remove_package(package)
    //                 .unwrap_or_else(|e| print_registry_error(e)),
    //             Ok(None) => println!("INFO: Registry does not contain given package"),
    //             Err(e) => println!("ERROR: <PACKAGE_PATH> does not point to a valid package"),
    //         },
    //         Err(e) => print_registry_error(e),
    //     },
    //     Command::Initialize => {
    //         if let Err(e) = Registry::initialize(&registry_path) {
    //             print_registry_error(e)
    //         }
    //     }
    //     Command::Download {
    //         package_location,
    //         package_root,
    //     } => match Registry::load(&registry_path) {
    //         Ok(mut registry) => match Package::download(package_location, &package_root) {
    //             Ok(package) => registry
    //                 .add_package(package)
    //                 .unwrap_or_else(|e| print_registry_error(e)),
    //             Err(e) => print_package_error(e),
    //         },
    //         Err(e) => print_registry_error(e),
    //     }, // Command::AddDependency {
    //        //     value,
    //        //     package_path,
    //        // } => {
    //        //     let registry = Registry::load(&registry_path);
    //        //     let found_package = registry.get_by_local_location(&package_path);
    //        //     match found_package {
    //        //         None => println!(
    //        //             "No package found in registry for path '{}'",
    //        //             package_path.display()
    //        //         ),
    //        //         Some(package) => {
    //        //             let dependency = Dependency::create(value);
    //        //             package.add_dependency(dependency)
    //        //         }
    //        //     }
    //        // }
    //        // Command::RemoveDependency {
    //        //     value,
    //        //     package_path,
    //        // } => {
    //        //     let registry = Registry::load(&registry_path);
    //        //     let found_package = registry.get_package_by_local_location(&package_path);
    //        //     match found_package {
    //        //         None => println!(
    //        //             "No package found in registry for path '{}'",
    //        //             package_path.display()
    //        //         ),
    //        //         Some(package) => {
    //        //             let dependency = Dependency::create(value);
    //        //             package.remove_dependency(&dependency)
    //        //         }
    //        //     }
    //        // }
    //        // Command::AddModule {
    //        //     module_path,
    //        //     identifier,
    //        // } => {
    //        //     let registry = Registry::load(&registry_path);
    //        //     let found_package = registry.get_package_by_local_location(&module_path);
    //        //     match found_package {
    //        //         None => println!(
    //        //             "No package found in registry for path '{}'",
    //        //             module_path.display()
    //        //         ),
    //        //         Some(package) => {
    //        //             let relative_module_path = package.strip_prefix(module_path);
    //        //             let module = Module::create(relative_module_path, identifier);
    //        //             package.add_module(module);
    //        //         }
    //        //     }
    //        // }
    //        // Command::RemoveModule { module_path } => {
    //        //     let registry = Registry::load(&registry_path);
    //        //     let found_package = registry.get_package_by_local_location(&module_path);
    //        //     match found_package {
    //        //         None => println!(
    //        //             "No package found in registry for path '{}'",
    //        //             module_path.display()
    //        //         ),
    //        //         Some(package) => {
    //        //             let relative_module_path = package.strip_prefix(&module_path);
    //        //             match package.get_module_by_location(relative_module_path) {
    //        //                 Some(module) => {
    //        //                     package.remove_module(&module);
    //        //                 }
    //        //                 None => println!(
    //        //                     "No module found with module path '{}'",
    //        //                     module_path.display()
    //        //                 ),
    //        //             }
    //        //         }
    //        //     }
    //        // }
    //        // Command::Search { modules } => {
    //        //     let registry = Registry::load(&registry_path);
    //        //     let i = registry.search_by_module_identifiers(&modules);
    //        //     for p in i {
    //        //         println!("{}", p)
    //        //     }
    //        // }
    // }
}
