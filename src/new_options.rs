use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub(crate) struct Cli {
    /// Specify the desired registry file.
    ///
    /// Use double quotes (") when path contains spaces or escape spaces
    /// Paths with environment variables are allowed
    /// Path has to point to a valid existing JSON file
    /// If the environment variable KNAPSAC_REGISTRY_PATH is set and this option is omitted $KNAPSAC_REGISTRY_PATH will be used instead
    ///
    /// [Examples]
    ///     ./knapsac -r /home/my_user/registry.json <SUBCOMMAND>
    ///     ./knapsac -r /home/my\ user/registry.json <SUBCOMMAND>
    ///     ./knapsac -r "/home/my user/registry.json" <SUBCOMMAND>
    ///     ./knapsac -r $HOME/registry.json <SUBCOMMAND>
    ///     ./knapsac <SUBCOMMAND> (only allowed when KNAPSAC_REGISTRY_PATH is set)
    ///
    /// [Caveats]
    /// Relative paths might work but are not supported
    ///
    /// [Environment]
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    #[clap(env = "KNAPSAC_REGISTRY_PATH")]
    #[clap(verbatim_doc_comment)]
    pub(crate) registry_path: PathBuf,

    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Adds a module to the registry
    ///
    /// [Examples]
    /// The following command:
    ///    
    ///     ./knapsac -r /home/my_user/registry.json add /home/my_user/file.txt
    ///
    /// will result in the following entry being added to the registry:
    ///
    ///     {
    ///         "local_location": "/home/my_user/file.txt"
    ///     }
    ///
    /// [Errors]
    /// An error is returned when:
    ///     the given <REGISTRY_PATH> does not point to a valid registry file
    #[clap(verbatim_doc_comment)]
    Add {
        /// Path to the module
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Path has to point to a file in a git repository
        ///
        /// [Examples]
        ///     /home/my_user/file.txt
        ///     /home/my\ user/file.txt
        ///     "/home/my user/file.txt"
        ///     $HOME/file.txt
        ///
        /// [Errors]
        /// An error is returned when:       
        ///     - the given <MODULE_PATH> does point to an existing module.
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        module_path: PathBuf,
    },
    /// Removes a module from the registry
    ///
    /// [Examples]    
    ///     ./knapsac -r /home/my_user/registry.json remove /home/my_user/file.txt
    ///
    /// will remove the following entry from the registry:
    ///
    ///     {
    ///         "local_location": "/home/my_user/file.txt"
    ///     }
    ///
    /// [Errors]
    /// An error is returned when:
    ///     the given <REGISTRY_PATH> does not point to a valid registry file
    ///
    #[clap(verbatim_doc_comment)]
    Remove {
        /// Path to the package
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Path has to point to a file in a git repository
        ///
        /// [Examples]
        ///     /home/my_user/file.txt
        ///     /home/my\ user/file.txt
        ///     "/home/my user/file.txt"
        ///     $HOME/file.txt
        ///
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        module_path: PathBuf,
    },
    /// Creates an empty registry at the given <REGISTRY_PATH>
    ///
    /// Overwrites any file already present
    #[clap(verbatim_doc_comment)]
    Initialize,
}
