// use crate::subcommands::add::Add;
// use crate::subcommands::dependencies::Dependencies;
// use crate::subcommands::remove::Remove;
// use clap::Args;
// use clap::Subcommand;
// use knapsac_lib::entry::Entry;
// use crate::subcommands::get::Get;
//
// #[derive(Args)]
// pub(crate) struct Module {
//     module_identifier: String,
//     #[clap(subcommand)]
//     command: ModuleCommand,
// }
//
// #[derive(Subcommand)]
// pub(crate) enum ModuleCommand {
//     Add(Add),
//     Dependencies(Dependencies),
//     Remove(Remove),
//     Get(Get),
// }
//
// impl Module {
//     pub(crate) fn handle_command(&self) {
//         let entry = Entry::StandaloneModule(self.module_identifier.clone());
//         match &self.command {
//             ModuleCommand::Add(a) => a.handle_command(entry),
//             ModuleCommand::Dependencies(d) => d.handle_command(entry),
//             ModuleCommand::Remove(r) => r.handle_command(entry),
//             ModuleCommand::Get(g) => g.handle_command(&entry),
//         }
//     }
// }
