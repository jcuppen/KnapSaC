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
    ///         "local_location": "/home/package/",
    ///         "remote_location": "<GIT_URL>"
    ///     }
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
        ///     the given <PACKAGE_PATH> does not point to a valid git repository
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        package_path: PathBuf,
    },
    /// Removes a package from the registry
    ///
    /// [Examples]    
    ///     ./knapsac -r /home/registry.json remove /home/package/file.txt
    ///
    /// will remove the following entry from the registry:
    ///
    ///     {
    ///         "local_location": "/home/package/",
    ///         "remote_location": "<GIT_URL>"
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
        ///     the given <PACKAGE_PATH> does not point to a valid git repository
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        package_path: PathBuf,
    },
    /// Creates an empty registry at the given <REGISTRY_PATH>
    ///
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
        /// Path to directory where the package must be downloaded to
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
        ///     the given <PACKAGE_ROOT> does not point to an existing directory
        ///     the given <PACKAGE_ROOT> points to a file
        ///
        /// [Environment]
        #[clap(env = "KNAPSAC_PACKAGE_ROOT")]
        #[clap(parse(from_os_str))]
        #[clap(verbatim_doc_comment)]
        package_root: PathBuf,
    },
    /// Adds a new dependency to the provided entry
    #[clap(verbatim_doc_comment)]
    AddDependency {
        /// Git Url of remote location of package
        ///
        /// The given <VALUE> must be a valid git url (http or https)
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
        /// Path to entry which the dependency should be added to
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
        ///     the given <PACKAGE_PATH> does not point to a valid git repository
        #[clap(verbatim_doc_comment)]
        package_path: PathBuf,
    },
    /// Remove a dependency for the provided entry
    RemoveDependency {
        /// Git Url of remote location of package
        ///
        /// The given <VALUE> must be a valid git url (http or https)
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
        ///     the given <PACKAGE_PATH> does not point to a valid git repository
        #[clap(verbatim_doc_comment)]
        package_path: PathBuf,
    },
    /// Add a module to the provided entry
    ///
    /// [Examples]
    /// Assuming the registry referenced by <REGISTRY_PATH> contains the following entry:
    ///
    ///     {
    ///         "local_location": "/home/package/",
    ///         "remote_location": "<GIT_URL>"
    ///     }
    ///
    /// the following command
    ///
    ///    ./knapsac -r /home/registry.json add-module /home/package/a.txt -i my_a
    ///
    /// will add this to the Package's manifest file
    ///
    ///     {
    ///         "identifier": "my_a",
    ///         "location": "a.txt"
    ///     }
    ///
    /// When the identifier is omitted the file stem will be used as the identifier instead.
    ///
    ///     ./knapsac -r /home/registry.json add-module /home/package/a.txt
    ///
    /// will add the following to the Package's manifest file
    ///
    ///     {
    ///         "identifier": "a",
    ///         "location": "a.txt"
    ///     }
    ///
    #[clap(verbatim_doc_comment)]
    AddModule {
        /// Path to module file that needs to be added
        /// KnapSaC will automatically attempt detect the relevant package
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Path has to point to a file in a git repository of a package that is in the registry
        /// Relative paths are allowed
        ///
        /// [Examples]
        ///     /home/my_user/package/file.txt
        ///     /home/my_user/package/some\ file.txt
        ///     "/home/my_user/package/some file.txt"
        ///
        /// [Errors]
        /// An error is returned when:
        ///     the given <MODULE_PATH> does not point to a valid git repository
        ///     the given <MODULE_PATH> does not point to a file
        ///     the given <MODULE_PATH> points to a Packages not in the registry pointed to by <REGISTRY_PATH>
        #[clap(verbatim_doc_comment)]
        module_path: PathBuf,
        /// Identifier for the module
        ///
        /// Use double quotes (") when the identifier contains spaces or escape spaces
        ///
        /// [Examples]
        ///     my_identifier
        ///     my\ identifier
        ///     "my identifier"
        ///
        #[clap(verbatim_doc_comment)]
        #[clap(short)]
        identifier: Option<String>,
    },
    /// Remove a module from the provided entry
    #[clap(verbatim_doc_comment)]
    RemoveModule {
        /// Path to module file that needs to be removed
        /// KnapSaC will automatically attempt detect the relevant package.
        ///
        /// Use double quotes (") when path contains spaces or escape spaces
        /// Paths with environment variables are allowed
        /// Path has to point to a file in a git repository of a package that is in the registry
        /// Relative paths are allowed
        ///
        /// [Examples]
        ///     /home/my_user/package/file.txt
        ///     /home/my_user/package/some\ file.txt
        ///     "/home/my_user/package/some file.txt"
        ///
        /// [Errors]
        /// An error is returned when:
        ///     the given <MODULE_PATH> does not point to a valid git repository
        ///     the given <MODULE_PATH> does not point to a file
        ///     the given <MODULE_PATH> points to a Packages not in the registry pointed to by <REGISTRY_PATH>
        #[clap(verbatim_doc_comment)]
        module_path: PathBuf,
    },
    Search {
        modules: Vec<String>,
    },
}
