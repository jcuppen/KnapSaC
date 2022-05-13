extern crate core;

use crate::new_options::{Cli, Command};
use knapsac_lib::error::{ModuleError, PackageError, RegistryError};
use std::path::PathBuf;
use std::process::exit;

// mod options;
mod new_options;

use clap::Parser;
use knapsac_lib::module::standalone_module::StandaloneModule;
use knapsac_lib::registry::get::FindBy;
use knapsac_lib::registry::Registry;

fn main() {
    let cli = Cli::parse();

    let registry_path: PathBuf = cli.registry_path;

    match cli.command {
        Command::Initialize => {
            if let Err(e) = Registry::initialize(&registry_path) {
                print_registry_error(e)
            }
        }
        Command::Add { module_path } => match Registry::load(registry_path) {
            Ok(mut r) => match StandaloneModule::create(module_path) {
                Ok(m) => r.add_module(m).unwrap_or_else(print_registry_error),
                Err(e) => print_module_error(e),
            },
            Err(e) => print_registry_error(e),
        },
        Command::Remove { module_path } => match Registry::load(registry_path) {
            Ok(mut r) => {
                match r.get_module(FindBy::LocalLocation(module_path)) {
                    None => println!("INFO: No such module found"),
                    Some(m) => r.remove_module(&m).unwrap_or_else(print_registry_error),
                };
            }
            Err(e) => print_registry_error(e),
        },
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

fn print_registry_error(e: RegistryError) {
    match e {
        RegistryError::RegistryPathNotAbsolute => {
            println!("ERROR: <REGISTRY_PATH> is not an absolute path")
        }
        RegistryError::RegistryPathNotJSON => {
            println!("ERROR: <REGISTRY_PATH> does not point to a JSON file")
        }
        RegistryError::RegistryPathNotFile => {
            println!("ERROR: <REGISTRY_PATH> does not point to a file")
        }
        RegistryError::NoRegistryFound => println!("ERROR: No registry found"),
        RegistryError::InvalidRegistry => println!("ERROR: Invalid registry found"),
    };
    exit(1);
}
fn print_package_error(e: PackageError) {
    match e {
        PackageError::NoRemoteLocation => {
            println!("ERROR: <PACKAGE_PATH> does not point to a package with a remote location")
        }
        PackageError::NotARepository => {
            println!("ERROR: <PACKAGE_PATH> does not point to a git repository")
        }
        PackageError::PackageRootNotADirectory => {
            println!("ERROR: <KNAPSAC_PACKAGE_ROOT> does not point to a directory")
        }
        PackageError::DownloadFailed => {
            println!("ERROR: Download of package located at <REMOTE_LOCATION> failed")
        }
    };
    exit(1)
}
fn print_module_error(e: ModuleError) {
    match e {
        ModuleError::LocationNotRelative => println!("ERROR: <MODULE_LOCATION> is not relative"),
        ModuleError::LocationNotAbsolute => println!("ERROR: <MODULE_LOCATION> is not absolute"),
        ModuleError::DoesNotExist => {
            println!("ERROR: <MODULE_LOCATION> does not point to existing file")
        }
    }
    exit(1)
}
