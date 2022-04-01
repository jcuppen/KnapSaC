use serde::Deserialize;
use serde::Serialize;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Deserialize, Serialize)]
pub(crate) struct Dependency {
    pub(crate) git_url: String,
}
