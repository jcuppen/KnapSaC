use crate::RegistrationStatus::{Known, Registered};
use fmt::Display;
use git2::string_array::StringArray;
use git2::Repository;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;
use std::path::PathBuf;

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
pub struct Module {
    registration_status: RegistrationStatus, //TODO: When is something registered
    pub(crate) path: PathBuf,
    local_location: PathBuf, //TODO: Where should this point towards?
    remote_location: Option<String>,
    dependency_file: PathBuf,
}

pub(crate) fn create_module(new_entry_path: PathBuf, repository: Repository) -> Module {
    let string_array = repository
        .remotes()
        .expect(&*format!("No remotes for: {}", repository.path().display()).as_str());

    if string_array.is_empty() {
        println!("No remotes!");
        create_known_module(new_entry_path, repository)
    } else {
        if string_array.len() == 1 {
            println!("Exactly 1 remote!");
        } else {
            println!("WARNING: Multiple remotes found using first for now!");
        }
        create_registered_module(new_entry_path, repository, string_array)
    }
}

fn create_registered_module(
    new_entry_path: PathBuf,
    repository: Repository,
    remotes: StringArray,
) -> Module {
    let remote_name_str = remotes.get(0).unwrap();
    let remote = repository.find_remote(remote_name_str).unwrap();

    Module {
        registration_status: Registered,
        path: new_entry_path.clone(),
        local_location: repository.path().to_path_buf(),
        remote_location: Some(String::from(remote.url().unwrap())), //FIX
        dependency_file: new_entry_path,                            //FIX
    }
}

fn create_known_module(new_entry_path: PathBuf, repository: Repository) -> Module {
    Module {
        registration_status: Known,
        path: new_entry_path.clone(),
        local_location: repository.path().to_path_buf(),
        remote_location: None,
        dependency_file: new_entry_path, //FIX
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
    }
}
