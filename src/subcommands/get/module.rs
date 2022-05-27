use std::process::exit;
use clap::Args;
use knapsac_lib2::registry::Registry;

#[derive(Args)]
/// Gets a module from the registry
///
/// [Examples]
/// The following command:
///
///     knapsac get module a
///
/// will result in the following entry being found in the registry if it exists:
///
///     {
///         "identifier": "a"
///         "output_directory": "/home/my_user/a/out"
///     }
///
/// [Errors]
/// An error is returned when:
///     the registry at `~/registry.json` is not valid
#[clap(verbatim_doc_comment)]
pub(crate) struct Module {
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

impl Module {
    pub(crate) fn handle_command(identifier: &str) {
        let r = Registry::load();
        if let Some(m) = r.get_module(identifier) {
            println!("{}", m.output_directory.display());
        } else {
            exit(1);
        }
    }
}
