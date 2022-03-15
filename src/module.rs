use crate::module::RegistrationStatus::{Known, Registered};
use fmt::Display;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;
use url::Url;

#[derive(Deserialize, Serialize)]
pub enum RegistrationStatus {
    Registered,
    Known,
}

#[derive(Deserialize, Serialize)]
pub struct Module {
    registration_status: RegistrationStatus,
    path: PathBuf,
    local_location: PathBuf,
    remote_location: Option<Url>,
    dependency_file: PathBuf,
}

pub(crate) fn create_registered_module(
    path: &str,
    package_location: &str,
    repository_location: &str,
    dependency_file: &str,
) -> Module {
    Module {
        registration_status: Registered,
        path: PathBuf::from(path),
        local_location: PathBuf::from(package_location),
        remote_location: Some(Url::parse(repository_location).unwrap()),
        dependency_file: PathBuf::from(dependency_file),
    }
}

pub(crate) fn create_known_module(
    path: &str,
    package_location: &str,
    dependency_file: &str,
) -> Module {
    Module {
        registration_status: Known,
        path: PathBuf::from(path),
        local_location: PathBuf::from(package_location),
        remote_location: None,
        dependency_file: PathBuf::from(dependency_file),
    }
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let remote_location = match &self.remote_location {
            None => "N/A".to_string(),
            Some(x) => x.as_str().to_string(),
        };
        let reg_status = match self.registration_status {
            RegistrationStatus::Registered => "Yes",
            RegistrationStatus::Known => "No",
        };

        writeln!(
            f,
            "{}\n\t-> Is Registered: {}\n\t-> {}\n\t-> {}\n\t-> {}",
            self.path.display(),
            reg_status,
            self.local_location.display(),
            remote_location,
            self.dependency_file.display()
        )
        // write!(f, "{}", self.path.to_str().unwrap());
        // write!(f, "\t-> {}", self.local_location.to_str().unwrap());

        // write!(f, "{}", i);
        // write!(f, "\t-> {}", self.remote_location.unwrap().as_str());
        // write!(f, "\t-> {}", self.dependency_file.to_str().unwrap())
    }
}

// struct UrlVisitor;

// impl<'de> Visitor<'de> for UrlVisitor {
//
// }
//
// impl Deserialize for Url {
//     fn deserialize<D>(deserializer: D) -> Result<Self, serde::de::Error> where D: Deserializer<'de> {
//         deserializer.deserialize_string()
//     }
// }

// impl Module {
//     pub(crate) fn dump(&self) {
//         match self {
//             Module::Registered {
//                 path,
//                 locations,
//                 dependency_file,
//             } => {
//                 println!("{}", path.to_str().unwrap());
//                 println!("\t-> {}", locations.local_location.to_str().unwrap());
//                 println!("\t-> {}", locations.remote_location.as_str());
//                 println!("\t-> {}", dependency_file.to_str().unwrap());
//             }
//             Module::Known {
//                 path,
//                 dependency_file,
//             } => {
//                 println!("{}", path.to_str().unwrap());
//                 println!("\t-> {}", dependency_file.to_str().unwrap());
//             }
//         }
//     }
// }

//     pub(crate) fn download(&self, target_parent: &PathBuf) {
//         let mut target_directory = target_parent.clone();
//         target_directory.push(&*self.id);
//
//         println!(
//             "Downloading `{}` to `{}`",
//             self.id,
//             target_directory.display()
//         );
//
//         Command::new("git")
//             .arg("clone")
//             .arg(&*self.location)
//             .arg(&target_directory)
//             .arg("--recurse-submodules")
//             .output()
//             .expect("failed to execute process");
//     }
//
//     pub(crate) fn build(&self, target_parent: &PathBuf) {
//         let mut build_directory = target_parent.clone();
//         build_directory.push(&*self.id);
//         build_directory.push("build");
//
//         println!("Creating build directory: `{}`", build_directory.display());
//
//         fs::create_dir(&build_directory).unwrap();
//
//         Command::new("cmake")
//             .current_dir(&build_directory)
//             .arg("..")
//             .output()
//             .expect("failed to execute process");
//         let output = Command::new("make")
//             .current_dir(&build_directory)
//             .arg("-j4")
//             .output()
//             .expect("failed to execute process");
//
//         io::stdout().write_all(&output.stdout).unwrap();
//         io::stderr().write_all(&output.stderr).unwrap();
//     }
// }

// impl ModuleSet {
//     pub(crate) fn empty(&self) -> bool {
//         return self.libraries.len() == 0;
//     }
//     pub(crate) fn add_library(&mut self, new_library: Library) {
//         self.libraries.push(new_library);
//     }
//     pub(crate) fn install(&self) {
//         let location = match options::get_options().location {
//             None => {
//                 let mut p = dirs::home_dir().unwrap();
//                 p.push("knapSaC_libraries");
//                 p
//             }
//             Some(loc) => loc.canonicalize().unwrap(),
//         };
//
//         println!(
//             "The requested packages will be downloaded to: `{}`!",
//             location.display()
//         );
//
//         for lib in self.libraries.clone() {
//             lib.download(&location);
//             lib.build(&location);
//         }
//     }
// }
