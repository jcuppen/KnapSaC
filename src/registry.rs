use crate::module::Module;
use crate::{create_module, options};
use fmt::Display;
use git2::Repository;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use std::fmt::Formatter;
use std::fs::write;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

#[derive(Deserialize, Serialize)]
pub(crate) struct Registry {
    #[serde(default = "default_registry_path")]
    #[serde(skip)]
    pub(crate) path: PathBuf,
    pub(crate) modules: Vec<Module>,
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

impl Display for Registry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let modules: String = self.modules.iter().map(|m| format!("{}", m)).collect();
        write!(f, "{}", modules)
    }
}

pub(crate) fn create_empty_registry(registry_path: &Path) -> Result<Registry> {
    Ok(Registry {
        path: registry_path.to_path_buf(),
        modules: vec![],
    })
}

impl Registry {
    pub(crate) fn save(&self) {
        let contents = serde_json::to_string(self).unwrap();
        write(self.path.as_path(), contents).unwrap()
    }

    pub(crate) fn add(&mut self, entry_path: PathBuf) {
        if self.modules.iter().any(|m| m.path == entry_path) {
            println!("WARNING: entry already in registry");
            println!("Entry will not be added to the registry");
            return;
        }

        let repository = Repository::discover(&entry_path).expect(&*format!(
            "Failed to discover repository @ {}",
            entry_path.display()
        ));

        self.modules.push(create_module(entry_path, repository));
    }

    pub(crate) fn remove(&mut self, entry_path: PathBuf) {
        let new_modules: Vec<Module> = self
            .modules
            .clone()
            .into_iter()
            .filter(|m| m.path != entry_path)
            .collect();
        self.modules = new_modules;
    }

    pub(crate) fn dump(&self) {
        println!("Registry used: '{}'", self.path.display());
        println!("Total number of entries: {}", self.modules.len());
        println!("{}", self);
    }
}
