pub(crate) mod add;
pub(crate) mod get;
pub(crate) mod remove;

// use crate::subcommands::MessageType::{Error, Info, Warning};

// use knapsac_lib::error::{ModuleError, PackageError, RegistryError};
// use std::fmt::{Display, Formatter};
// use std::path::Path;
// use std::process::exit;
/*
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

    fn print_load_registry_message(registry_path: &Path) {
        Self::print_message(format!(
            "Loading registry located @ {}",
            registry_path.display()
        ));
    }

    fn print_search_module_message(output_path: &Path) {
        Self::print_message(format!(
            "Searching for module with <OUTPUT_PATH> ({})",
            output_path.display()
        ));
    }
    fn print_module_not_found_message(output_path: &Path) {
        Self::print_message(format!(
            "No module found in registry @ <OUTPUT_PATH> ({})",
            output_path.display()
        ));
    }
    fn print_no_such_dependency_message(output_path: &Path, identifier: &str) {
        Self::print_message(format!(
            "No dependency found for identifier '{}' for module with <OUTPUT_PATH> ({})",
            identifier,
            output_path.display()
        ));
    }

    fn print_registry_error(
        e: RegistryError,
        registry_path: &Path,
        output_path: Option<&Path>,
    ) -> ! {
        let msg = match e {
            RegistryError::RegistryPathNotAbsolute => {
                format!(
                    "<REGISTRY_PATH> ({}) is not an absolute path",
                    registry_path.display()
                )
            }
            RegistryError::RegistryPathNotJSON => {
                format!(
                    "<REGISTRY_PATH> ({}) does not point to a JSON file",
                    registry_path.display()
                )
            }
            RegistryError::RegistryPathNotFile => {
                format!(
                    "<REGISTRY_PATH> ({})' does not point to a file",
                    registry_path.display()
                )
            }
            RegistryError::NoRegistryFound => {
                format!(
                    "No registry found at <REGISTRY_PATH> ({}). Please create one first with  the `initialize` command",
                    registry_path.display()
                )
            }
            RegistryError::InvalidRegistry => {
                format!(
                    "Invalid registry found at <REGISTRY_PATH> ({})",
                    registry_path.display()
                )
            }
            RegistryError::ModuleAlreadyInRegistry => {
                format!(
                    "The registry at <REGISTRY_PATH> ({}) already contains an entry with <MODULE_PATH> ({})",
                    registry_path.display(),
                    output_path.unwrap().display()
                )
            }
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
            ModuleError::SourceLocationNotRelative => "<MODULE_LOCATION> is not relative",
            ModuleError::OutputLocationNotRelative => "<OUTPUT_LOCATION> is not relative",
            ModuleError::SourceLocationNotAbsolute => "<MODULE_LOCATION> is not absolute",
            ModuleError::OutputLocationNotAbsolute => "<OUTPUT_LOCATION> is not absolute",
            ModuleError::SourceLocationDoesNotExist => "<MODULE_LOCATION> does not point to existing file",
            ModuleError::OutputLocationDoesNotExist => "<OUTPUT_LOCATION> does not point to existing file",
            ModuleError::InvalidManifest => "Module manifest is not a valid manifest file",
            ModuleError::CyclicDependency => "Cyclic dependency detected",
        };
        Self::print_error(msg)
    }
}
 */
