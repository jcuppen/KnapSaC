use crate::module::{create_known_module, create_registered_module, Module};
use crate::options;
use fmt::Display;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufWriter, Write};
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
    options::get_options().registry_filepath
}

pub(crate) fn load_registry(registry_path: &Path) -> Result<Registry> {
    let data = match fs::read_to_string(registry_path) {
        Ok(d) => d,
        Err(_) => {
            println!("Unable to read file '{}'", registry_path.display());
            println!("creating an empty registry instead.");
            return create_empty_registry(registry_path);
        }
    };
    serde_json::from_str(&*data)
}

impl Display for Registry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let modules = self
            .modules
            .iter()
            .map(|m| format!("{}", m))
            .collect::<String>();
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
    pub(crate) fn save(&self, registry_path: &Path) {
        let file = match File::open(registry_path) {
            Ok(f) => f,
            Err(_) => match File::create(registry_path) {
                Ok(f) => f,
                Err(e) => panic!("Could not create file {}: {}", registry_path.display(), e),
            },
        };

        let mut buf_writer = BufWriter::new(file);
        let contents = serde_json::to_string(self);
        match buf_writer.write_all(contents.unwrap().as_bytes()) {
            Ok(_) => {}
            Err(e) => panic!(
                "Could not write registry to file {}: {}",
                registry_path.display(),
                e
            ),
        }
    }

    pub(crate) fn dump(&self) {
        println!("Registry used: '{}", self.path.display());
        println!("Total number of entries: {}", self.modules.len());
        println!("{}", self);
    }
}
