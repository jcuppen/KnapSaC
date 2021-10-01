#[derive(Clone)]
pub(crate) struct Package {
    pub(crate) id: String,
    pub(crate) location: String,
}

pub(crate) fn create_package(id: &str, location: &str) -> Package {
    Package {
        id: String::from(id),
        location: String::from(location),
    }
}
