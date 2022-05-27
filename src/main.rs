extern crate core;

mod new_options;
mod subcommands;

use crate::new_options::{Cli, Command};
use crate::subcommands::add::AddCommand;
use crate::subcommands::remove::RemoveCommand;
use crate::subcommands::{add, get, remove};
use crate::subcommands::get::GetCommand;
use clap::Parser;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Command::Add(a) => match a.command {
            AddCommand::Module(args) => {
                add::module::Module::handle_command(args.identifier, args.output_path)
            }
            AddCommand::Dependency(args) => add::dependency::Dependency::handle_command(
                &args.module_identifier,
                &args.dependency_identifier,
            ),
        },
        Command::Remove(r) => match r.command {
            RemoveCommand::Module(args) => {
                remove::module::Module::handle_command( &args.identifier)
            }
        },
        Command::Get(g) => match g.command {
            GetCommand::Dependency(args) => get::dependency::Dependency::handle_command(
                &args.module_identifier,
                &args.dependency_identifier,
            ),
            GetCommand::Module(args) => get::module::Module::handle_command(&args.identifier),
        },
    }
    std::process::exit(0);

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
