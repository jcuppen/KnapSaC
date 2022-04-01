use crate::dependency::Dependency;
use crate::package::RegistrationStatus::{Known, Registered};
use git2::string_array::StringArray;
use git2::Repository;
use serde::Deserialize;
use serde::Serialize;

use std::fmt::{Display, Formatter};
use std::fs::write;
use std::path::PathBuf;
use std::{fmt, fs};

#[derive(Deserialize, Serialize, Clone)]
pub enum RegistrationStatus {
    Registered,
    Known,
}

/*
 Questions:
 - When is something considered Registered?
 - Does a packages always have to belong to a git-repository?
    - If not, where should 'local_location' point to in the case there is no git repository.
 - Is the dependency file mandatory?
*/
#[derive(Deserialize, Serialize, Clone)]
pub struct Package {
    registration_status: RegistrationStatus,
    pub(crate) local_location: PathBuf,
    pub(crate) remote_location: Option<String>,
    manifest_location: PathBuf,
}

pub(crate) fn create_package(local_repository_root: PathBuf, repository: Repository) -> Package {
    let mut manifest_location = local_repository_root.clone();
    manifest_location.push("dependencies");
    manifest_location.set_extension("json");

    if !manifest_location.exists() {
        panic!("No manifest found @ {}", local_repository_root.display());
    }

    let string_array = repository.remotes().expect(&*format!(
        "No remotes for: {}",
        local_repository_root.display()
    ));

    if string_array.is_empty() {
        println!("No remotes!");
        return create_known_package(local_repository_root, manifest_location);
    }

    if string_array.len() == 1 {
        println!("Exactly 1 remote!");
    } else {
        println!("WARNING: Multiple remotes found using first for now!");
    }

    create_registered_package(
        local_repository_root,
        repository,
        string_array,
        manifest_location,
    )
}

fn create_registered_package(
    local_repository_root: PathBuf,
    repository: Repository,
    remotes: StringArray,
    manifest_location: PathBuf,
) -> Package {
    let remote_name_str = remotes.get(0).unwrap();
    let remote = repository.find_remote(remote_name_str).unwrap();

    Package {
        registration_status: Registered,
        local_location: local_repository_root,
        remote_location: Some(String::from(remote.url().unwrap())), //FIX
        manifest_location,
    }
}

fn create_known_package(local_repository_root: PathBuf, manifest_location: PathBuf) -> Package {
    Package {
        registration_status: Known,
        local_location: local_repository_root,
        remote_location: None,
        manifest_location,
    }
}

impl Package {
    pub(crate) fn add_dependency(&self, value: String) {
        let new_dep = Dependency { git_url: value };
        if let Ok(data) = fs::read_to_string(self.manifest_location.clone()) {
            let mut dependencies: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
            dependencies.push(new_dep);
            dependencies.sort();
            dependencies.dedup();
            let contents = serde_json::to_string(&dependencies).unwrap();
            write(self.manifest_location.as_path(), contents).unwrap()
        }
    }

    pub(crate) fn remove_dependency(&self, value: String) {
        let dep_to_remove = Dependency { git_url: value };
        if let Ok(data) = fs::read_to_string(self.manifest_location.clone()) {
            let mut dependencies: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
            if let Some(index) = dependencies.iter().position(|d| d == &dep_to_remove) {
                dependencies.remove(index);
            }
            let contents = serde_json::to_string(&dependencies).unwrap();
            write(self.manifest_location.as_path(), contents).unwrap()
        }
    }

    pub(crate) fn dump(&self, list_dependencies: bool) {
        println!("{}", self);

        if list_dependencies {
            if let Ok(data) = fs::read_to_string(self.manifest_location.clone()) {
                let i: Vec<Dependency> = serde_json::from_str(&*data).unwrap();
                println!("{:?}", i);
            }
        }
    }
}

impl Display for Package {
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
            "{}\n\t-> Is Registered: {}\n\t-> {}\n\t-> {}",
            self.local_location.display(),
            reg_status,
            remote_location,
            self.manifest_location.display()
        )
    }
}
