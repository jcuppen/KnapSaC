mod new_options;
mod subcommands;

use crate::new_options::{Cli, Command};
use clap::Parser;
use std::process::exit;

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Command::Add(a) => a.handle_command(),
        Command::Has(h) => h.handle_command(),
        Command::Get(g) => g.handle_command(),
        Command::Search(s) => s.handle_command(),
        Command::MarkAsModule(m) => m.handle_command(),
        Command::Remove(r) => r.handle_command(),
        Command::Dependencies(d) => d.handle_command(),
        Command::Package(p) => p.handle_command(),
    }
    exit(0);
}
