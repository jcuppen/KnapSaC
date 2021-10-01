use crate::package::Package;

pub(crate) struct Registry {
    packages: Vec<Package>,
}

pub(crate) fn create_registry(packages: Vec<Package>) -> Registry {
    Registry { packages }
}

impl Registry {
    pub(crate) fn check_availability(&self, identifier: String) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|x| x.id.to_lowercase() == identifier.to_lowercase())
            .collect()
    }
}
