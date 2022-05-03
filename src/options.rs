use clap::{Parser, Subcommand};
use std::path::PathBuf;
use url::Url;

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
    /// Adds a package to the registry
    ///
    /// [Examples]
    /// It is assumed there is a git repository on disk located at '/home/my_user/package/'
    ///    
    ///     ./knapsac -r /home/registry.json add /home/package/file.txt
    ///
    /// will result in the following entry being added to the registry:
    ///
    ///     {
    ///         "registration_status": "<REGISTRATION_STATUS>",
    ///         "local_location": "/home/package/",
    ///         "remote_location": "<GIT_URL>"
    ///     }
    ///
    /// <REGISTRATION_STATUS> can be
    ///     Registered: when the discovered git repository has a remote
    ///     Known: when the discovered git repository has no remotes
    ///
    /// [Errors]
    /// An error is returned when:
    ///     the given <REGISTRY_PATH> does not point to a valid registry file
    #[clap(verbatim_doc_comment)]
    Add {
        /// Path to the package
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Path has to point to a file in a git repository
        /// Relative paths are allowed
        ///
        /// [Examples]
        ///     /home/my_user/package/
        ///     /home/my_user/package/file.txt
        ///     /home/my_user/package/nested/dir
        ///     /home/my_user/package/some\ file.txt
        ///     "/home/my_user/package/some file.txt"
        ///     $HOME/package/
        ///
        /// [Errors]
        /// An error is returned when:       
        ///     the given <PACKAGE_LOCATION> does not point to a valid git repository
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        package_location: PathBuf,
    },
    /// Removes a package from the registry
    ///
    /// [Examples]    
    ///     ./knapsac -r /home/registry.json remove /home/package/file.txt
    ///
    /// will remove the following entry from the registry:
    ///
    ///     {
    ///         "registration_status": "<REGISTRATION_STATUS>",
    ///         "local_location": "/home/package/",
    ///         "remote_location": "<GIT_URL>"
    ///     }
    ///
    /// <REGISTRATION_STATUS> can be
    ///     Registered: when the discovered git repository has a remote
    ///     Known: when the discovered git repository has no remotes
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
        /// Relative paths are allowed
        ///
        /// [Examples]
        ///     /home/my_user/package/
        ///     /home/my_user/package/file.txt
        ///     /home/my_user/package/nested/dir
        ///     /home/my_user/package/some\ file.txt
        ///     "/home/my_user/package/some file.txt"
        ///     $HOME/package/
        ///
        /// [Errors]
        /// An error is returned when:       
        ///     the given <PACKAGE_LOCATION> does not point to a valid git repository
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        package_location: PathBuf,
    },
    /// Creates an empty registry at the given <REGISTRY_PATH>
    /// Overwrites any file already present
    #[clap(verbatim_doc_comment)]
    Initialize,
    /// Downloads the package and adds it to the registry
    ///
    /// [Examples]    
    ///     ./knapsac -r /home/registry.json download https://example.com/user/package /home/my_user/package_root    
    ///
    /// will download the package located at 'https://example.com/user/package'
    /// and store it in a random subdirectory of '/home/my_user/package_root/`
    /// (e.g. `/home/my_user/package_root/Yo1Tr9F3iF-LFHX9i9GvA')
    ///
    /// Then the packages will be registered in the registry at `/home/registry.json`
    ///
    /// [Errors]
    /// An error is returned when:
    ///     the given <REGISTRY_PATH> does not point to a valid registry file
    #[clap(verbatim_doc_comment)]
    Download {
        /// Git Url of remote location of package
        ///
        /// The given <PACKAGE_LOCATION> must be a valid git url (http or https)
        ///
        /// [Examples]
        ///     http://example.com/user/package
        ///     http://example.com/user/package.git
        ///     https://example.com/user/package
        ///     https://example.com/user/package.git
        ///
        /// [Caveats]
        /// Only supports 'http' and 'https' urls
        /// SSH is currently not supported
        #[clap(verbatim_doc_comment)]
        package_location: Url,
        /// Path to where package must be downloaded to
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Relative paths are allowed
        /// If the environment variable KNAPSAC_PACKAGE_ROOT is set and this option is omitted $KNAPSAC_PACKAGE_ROOT will be used instead
        ///
        /// [Examples]
        ///     /home/my_user/package_root/
        ///     /home/my_user/package\ root/
        ///     "/home/my_user/package root/"
        ///     $HOME/package_root/
        ///
        /// [Errors]
        /// An error is returned when:       
        ///     the given <TARGET_LOCATION> does not point to an existing directory
        ///     the given <TARGET_LOCATION> points to a file
        ///
        /// [Environment]
        #[clap(env = "KNAPSAC_PACKAGE_ROOT")]
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        target_location: PathBuf,
    },
    /// Adds a new dependency to the provided entry
    ///
    #[clap(verbatim_doc_comment)]
    AddDependency {
        /// Git Url of remote location of package
        ///
        /// The given <PACKAGE_LOCATION> must be a valid git url (http or https)
        ///
        /// [Examples]
        ///     http://example.com/user/package
        ///     http://example.com/user/package.git
        ///     https://example.com/user/package
        ///     https://example.com/user/package.git
        ///
        /// [Caveats]
        /// Only supports 'http' and 'https' urls
        /// SSH is currently not supported
        #[clap(short)]
        #[clap(verbatim_doc_comment)]
        value: Url,
        /// Path to entry where the dependency should be added
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Path has to point to a file in a git repository
        /// Relative paths are allowed
        ///
        /// [Examples]
        ///     /home/my_user/package/
        ///     /home/my_user/package/file.txt
        ///     /home/my_user/package/nested/dir
        ///     /home/my_user/package/some\ file.txt
        ///     "/home/my_user/package/some file.txt"
        ///     $HOME/package/
        ///
        /// [Errors]
        /// An error is returned when:
        ///     the given <PACKAGE_LOCATION> does not point to a valid git repository
        #[clap(verbatim_doc_comment)]
        package_location: PathBuf,
    },
    /// Remove a dependency for the provided entry
    RemoveDependency {
        /// Git Url of remote location of package
        ///
        /// The given <PACKAGE_LOCATION> must be a valid git url (http or https)
        ///
        /// [Examples]
        ///     http://example.com/user/package
        ///     http://example.com/user/package.git
        ///     https://example.com/user/package
        ///     https://example.com/user/package.git
        ///
        /// [Caveats]
        /// Only supports 'http' and 'https' urls
        /// SSH is currently not supported
        #[clap(short)]
        #[clap(verbatim_doc_comment)]
        value: Url,
        /// Path to entry where the dependency should be removed
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Path has to point to a file in a git repository
        /// Relative paths are allowed
        ///
        /// [Examples]
        ///     /home/my_user/package/
        ///     /home/my_user/package/file.txt
        ///     /home/my_user/package/nested/dir
        ///     /home/my_user/package/some\ file.txt
        ///     "/home/my_user/package/some file.txt"
        ///     $HOME/package/
        ///
        /// [Errors]
        /// An error is returned when:
        ///     the given <PACKAGE_LOCATION> does not point to a valid git repository
        #[clap(verbatim_doc_comment)]
        package_location: PathBuf,
    },
    // /// Check if local registry contains a certain entry
    // Contains(SearchVariant),
}
