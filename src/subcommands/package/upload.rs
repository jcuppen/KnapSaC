use knapsac_lib::registry::Registry;
use clap::Args;
use url::Url;

#[derive(Args)]
pub(crate) struct Upload {
    #[clap(short)]
    git_url: Option<Url>
}

impl Upload {
    pub(crate) fn handle_command(&self, identifier: &str) {
        let mut r = Registry::load();
        r.upload(identifier, self.git_url.clone());
    }
}
