use std::path::PathBuf;

pub(crate) enum Entry {
    Executable(PathBuf),
    StandaloneModule(String),
    PackageModule,
}
