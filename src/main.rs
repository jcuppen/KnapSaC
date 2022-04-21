extern crate core;

mod dependency;
mod input;
mod module;
mod options;
mod package;
mod registry;
mod util;

use crate::options::{Command, SearchVariant};
use crate::registry::load_registry;

fn main() {
    let registry_path = options::get_options().registry_path;

    // Load registry
    let mut registry = load_registry(&registry_path).expect(&*format!(
        "Failed to parse registry @ {}",
        registry_path.display()
    ));

    match options::get_options().command {
        Command::Add { path } => {
            registry.add(&path);
        }
        Command::AddDependency { 
            path, value } => registry.add_dependency(&path, value),
        Command::RemoveDependency { path, value } => registry.remove_dependency(&path, value),
        Command::Remove { path } => {
            registry.remove(&path);
        }
        Command::Dump {
            entry,
            list_dependencies,
        } => registry.dump(entry, list_dependencies),
        Command::Contains(variant) => match variant {
            SearchVariant::Local { path } => {
                if registry.find_entry_by_local_location(&path).is_some() {
                    println!("Found");
                } else {
                    println!("Not Found");
                }
            }
            SearchVariant::Remote { git_url } => {
                if registry.find_entry_by_remote_location(&git_url).is_some() {
                    println!("Found")
                } else {
                    println!("Not Found")
                }
            }
        },
    }
    registry.save();
}
