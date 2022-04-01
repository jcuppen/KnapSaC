use crate::options;
use crate::package::{create_package, Package};
use crate::util::{discover_git_repository, infer_working_directory};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use std::fs;
use std::fs::write;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub(crate) struct Registry {
    #[serde(default = "default_registry_path")]
    #[serde(skip)]
    path: PathBuf,
    packages: Vec<Package>,
}

fn default_registry_path() -> PathBuf {
    options::get_options().registry_path
}

pub(crate) fn load_registry(registry_path: &Path) -> Result<Registry> {
    if let Ok(data) = fs::read_to_string(registry_path) {
        return serde_json::from_str(&*data);
    }
    println!("Unable to read file '{}'", registry_path.display());
    println!("creating an empty registry instead.");
    create_empty_registry(registry_path)
}

pub(crate) fn create_empty_registry(registry_path: &Path) -> Result<Registry> {
    Ok(Registry {
        path: registry_path.to_path_buf(),
        packages: vec![],
    })
}

impl Registry {
    fn find_entry(&self, entry_path: &Path) -> &Package {
        let inferred_working_directory = infer_working_directory(entry_path);
        return self
            .packages
            .iter()
            .find(|p| p.local_location == inferred_working_directory)
            .expect(&*format!(
                "No package found @ {}",
                inferred_working_directory.display()
            ));
    }

    pub(crate) fn save(&self) {
        let contents = serde_json::to_string(self).unwrap();
        write(self.path.as_path(), contents).unwrap()
    }

    pub(crate) fn add(&mut self, entry_path: &Path) {
        let local_repository_root = infer_working_directory(entry_path);

        if self
            .packages
            .iter()
            .any(|m| m.local_location == local_repository_root)
        {
            println!("WARNING: entry already in registry");
            println!("Entry will not be added to the registry");
            return;
        }

        let repository = discover_git_repository(entry_path);

        self.packages
            .push(create_package(local_repository_root, repository));
    }

    pub(crate) fn add_dependency(&self, entry_path: &Path, value: i32) {
        let found_package = self.find_entry(entry_path);
        found_package.add_dependency(value);
    }

    pub(crate) fn remove_dependency(&self, entry_path: &Path, value: i32) {
        let found_package = self.find_entry(entry_path);
        found_package.remove_dependency(value);
    }

    pub(crate) fn remove(&mut self, entry_path: &Path) {
        let local_repository_root = infer_working_directory(entry_path);
        let new_modules: Vec<Package> = self
            .packages
            .clone()
            .into_iter()
            .filter(|p| p.local_location != local_repository_root)
            .collect();
        self.packages = new_modules;
    }

    pub(crate) fn dump(&self, entry: Option<PathBuf>, list_dependencies: bool) {
        if let Some(e) = entry {
            let found_package = self.find_entry(&e);
            println!("Registry used: '{}'", self.path.display());
            found_package.dump(list_dependencies);
            return;
        }

        println!("Registry used: '{}'", self.path.display());
        println!("Total number of entries: {}", self.packages.len());
        self.packages.iter().for_each(|p| p.dump(list_dependencies));
    }
}
