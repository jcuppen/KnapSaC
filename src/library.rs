use crate::options;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Library {
    pub(crate) id: String,
    pub(crate) location: String,
    pub(crate) modules: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct LibrarySet {
    pub(crate) libraries: Vec<Library>,
}

pub(crate) fn create_library(id: &str, location: &str, modules: Vec<&str>) -> Library {
    Library {
        id: String::from(id),
        location: String::from(location),
        modules: modules.iter().map(|&x| String::from(x)).collect(),
    }
}

impl Library {
    pub(crate) fn download(&self, target_parent: &PathBuf) {
        let mut target_directory = target_parent.clone();
        target_directory.push(&*self.id);

        println!(
            "Downloading `{}` to `{}`",
            self.id,
            target_directory.display()
        );

        Command::new("git")
            .arg("clone")
            .arg(&*self.location)
            .arg(&target_directory)
            .arg("--recurse-submodules")
            .output()
            .expect("failed to execute process");
    }

    pub(crate) fn build(&self, target_parent: &PathBuf) {
        let mut build_directory = target_parent.clone();
        build_directory.push(&*self.id);
        build_directory.push("build");

        println!("Creating build directory: `{}`", build_directory.display());

        fs::create_dir(&build_directory).unwrap();

        Command::new("cmake")
            .current_dir(&build_directory)
            .arg("..")
            .output()
            .expect("failed to execute process");
        let output = Command::new("make")
            .current_dir(&build_directory)
            .arg("-j4")
            .output()
            .expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}

impl LibrarySet {
    pub(crate) fn empty(&self) -> bool {
        return self.libraries.len() == 0;
    }
    pub(crate) fn add_library(&mut self, new_library: Library) {
        self.libraries.push(new_library);
    }
    pub(crate) fn install(&self) {
        let location = match options::get_options().location {
            None => {
                let mut p = dirs::home_dir().unwrap();
                p.push("knapSaC_libraries");
                p
            }
            Some(loc) => loc.canonicalize().unwrap(),
        };

        println!(
            "The requested packages will be downloaded to: `{}`!",
            location.display()
        );

        for lib in self.libraries.clone() {
            lib.download(&location);
            lib.build(&location);
        }
    }
}
