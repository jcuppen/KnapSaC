extern crate core;

mod new_options;
mod subcommands;

use crate::new_options::{Cli, Command};
use clap::Parser;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Command::Module(m) => m.handle_command(),
        Command::Executable(e) => e.handle_command(),
    }
    std::process::exit(0);
}
