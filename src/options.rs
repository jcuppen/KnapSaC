use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
pub(crate) struct Cli {
    /// Specify the desired registry file
    #[clap(short = 'R', long)]
    #[clap(parse(from_os_str))]
    #[clap(env = "KNAPSAC_REGISTRY_PATH")]
    pub(crate) registry_path: PathBuf,

    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Adds a package to the registry
    Add {
        /// Path to the package
        #[clap(parse(from_os_str))]
        package_location: PathBuf,
    },
    /// Removes a package from the registry
    Remove {
        /// Path to the package
        #[clap(parse(from_os_str))]
        package_location: PathBuf,
    },
    /// Creates an empty registry
    Initialize,
    /// Downloads the package and adds it to the registry
    Download {
        /// Git Url of remote location of package
        package_location: Url,
        /// Path where package must be downloaded to
        #[clap(env = "KNAPSAC_PACKAGE_ROOT")]
        #[clap(parse(from_os_str))]
        target_location: PathBuf,
    },
    // /// Adds a new dependency to the provided entry
    // AddDependency {
    //     #[structopt(short = "v")]
    //     value: String,
    //     /// Path to entry where the dependency should be added
    //     path: PathBuf,
    // },
    // /// Remove a dependency for the provided entry
    // RemoveDependency {
    //     #[structopt(short = "v")]
    //     value: String,
    //     /// Path to entry where the dependency should be removed
    //     path: PathBuf,
    // },
    // /// Check if local registry contains a certain entry
    // Contains(SearchVariant),
}

// #[derive(StructOpt)]
// pub(crate) enum SearchVariant {
//     Local { path: PathBuf },
//     Remote { git_url: String },
// }
