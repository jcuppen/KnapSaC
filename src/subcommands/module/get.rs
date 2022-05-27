use clap::Args;
use knapsac_lib::registry::Registry;
use std::process::exit;

#[derive(Args)]
/// Gets a module from the registry
///
/// [Examples]
/// The following command:
///
///     knapsac module get a
///
/// will return the <OUTPUT_PATH> of module 'b'
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct Get {
    /// Identifier for the module
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    ///
    /// [Examples]
    ///     my_a
    ///     "my a"
    ///     my\ a
    #[clap(verbatim_doc_comment)]
    pub(crate) identifier: String,
}

pub(crate) fn handle_command(identifier: &str) {
    let r = Registry::load();
    if let Some(m) = r.get_module(identifier) {
        println!("{}", m.output_path.display());
    } else {
        exit(1);
    }
}
