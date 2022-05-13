pub(crate) mod add;
pub(crate) mod initialize;
pub(crate) mod remove;
pub(crate) mod add_dependency;

use knapsac_lib::error::{ModuleError, PackageError, RegistryError};
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::process::exit;
use crate::subcommands::MessageType::{Error, Info, Warning};

enum MessageType {
    Info,
    Warning,
    Error,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Info => "Info",
            Warning => "Warning",
            Error => "Error",
        };
        write!(f, "{}", str)
    }
}

fn print_msg<S: Display>(t: MessageType, msg: S) {
    match t {
        Error => eprintln!("[{}]: {}", t, msg),
        _ => println!("[{}]: {}", t, msg),
    };
}

pub trait Subcommand {
    fn print_message<S: Display>(msg: S) {
        print_msg(Info, msg);
    }

    fn print_warning<S: Display>(msg: S) {
        print_msg(Warning, msg);
    }

    fn print_error<S: Display>(msg: S) -> ! {
        print_msg(Error, msg);
        exit(1)
    }

    fn print_load_registry_message(registry_path: &PathBuf) {
        Self::print_message(format!(
            "Loading registry located @ {}",
            registry_path.display()
        ));
    }

    fn print_search_module_message(module_path: &Path) {
        Self::print_message(format!("Searching for module located @ {}", module_path.display()));
    }
    fn print_module_not_found_message(module_path: &Path) {
        Self::print_message(format!("No module found in registry that was located @ {}", module_path.display()));
    }

    fn print_registry_error<P: AsRef<Path>>(e: RegistryError, registry_path: P) -> ! {
        let msg = match e {
            RegistryError::RegistryPathNotAbsolute => {
                "<REGISTRY_PATH> is not an absolute path".to_string()
            }
            RegistryError::RegistryPathNotJSON => {
                "<REGISTRY_PATH> does not point to a JSON file".to_string()
            }
            RegistryError::RegistryPathNotFile => {
                "<REGISTRY_PATH> does not point to a file".to_string()
            }
            RegistryError::NoRegistryFound => {
                format!(
                    "No registry found @ {}. Please create one first with  the `initialize` command",
                    registry_path.as_ref().display()
                )
            }
            RegistryError::InvalidRegistry => "Invalid registry found".to_string(),
        };
        Self::print_error(msg)
    }

    fn print_package_error(e: PackageError) -> ! {
        let msg = match e {
            PackageError::NoRemoteLocation => {
                "<PACKAGE_PATH> does not point to a package with a remote location"
            }
            PackageError::NotARepository => "<PACKAGE_PATH> does not point to a git repository",
            PackageError::PackageRootNotADirectory => {
                "<KNAPSAC_PACKAGE_ROOT> does not point to a directory"
            }
            PackageError::DownloadFailed => {
                "Download of package located at <REMOTE_LOCATION> failed"
            }
            PackageError::InvalidManifest => "Package Manifest was not a valid manifest",
        };
        Self::print_error(msg)
    }
    fn print_module_error(e: ModuleError) -> ! {
        let msg = match e {
            ModuleError::LocationNotRelative => "<MODULE_LOCATION> is not relative",
            ModuleError::LocationNotAbsolute => "<MODULE_LOCATION> is not absolute",
            ModuleError::DoesNotExist => "<MODULE_LOCATION> does not point to existing file",
            ModuleError::InvalidManifest => "Module manifest is not a valid manifest file",
            ModuleError::CyclicDependency => "Cyclic dependency detected",
        };
        Self::print_error(msg)
    }
}
