use clap::ArgEnum;
use clap::Args;
use knapsac_lib::registry::Registry;
use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, ArgEnum)]
pub(crate) enum ItemType {
    Module,
    Executable,
}
#[derive(Args)]
pub(crate) struct BySource {
    #[clap(arg_enum)]
    #[clap(short)]
    item_type: Option<ItemType>,
    source_file: PathBuf,
}

impl BySource {
    pub(crate) fn handle_command(&self) {
        let r = Registry::load();
        let source_file_path = self.source_file.canonicalize().unwrap();
        let b = match self.item_type {
            None => r.has_item_source(&source_file_path),
            Some(ItemType::Module) => r.has_module_source(&source_file_path),
            Some(ItemType::Executable) => r.has_executable_source(&source_file_path),
        };

        if !b {
            exit(1);
        }
    }
}
