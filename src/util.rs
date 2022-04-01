use git2::Repository;
use std::path::{Path, PathBuf};

pub(crate) fn discover_git_repository(path: &Path) -> Repository {
    Repository::discover(path).expect(&*format!(
        "Failed to discover repository @ {}",
        path.display()
    ))
}

pub(crate) fn infer_working_directory(path: &Path) -> PathBuf {
    discover_git_repository(path)
        .workdir()
        .expect(&*format!(
            "Failed to find root of local repository for path '{}'",
            path.display(),
        ))
        .to_path_buf()
}
