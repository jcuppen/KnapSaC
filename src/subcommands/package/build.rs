// use knapsac_lib::registry::Registry;
// use clap::Args;
//
// #[derive(Args)]
// pub(crate) struct Build {
//     command_name: String,
//     out_option: String,
// }
//
// impl Build {
//     pub(crate) fn handle_command(&self, identifier: &str) {
//         let r = Registry::load();
//         r.build_package(identifier, &self.command_name, &self.out_option);
//
//         println!("Please recompile all package modules of package '{}'", identifier);
//     }
// }
