// use std::io;
//
// pub(crate) enum BooleanInput {
//     Yes,
//     No,
//     Invalid,
// }
//
// pub(crate) fn parse() -> BooleanInput {
//     let mut input = String::new();
//
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//
//     if input == "\n" {
//         return BooleanInput::Yes;
//     }
//     match input.trim() {
//         "Y" | "y" | "Yes" | "yes" => BooleanInput::Yes,
//         "N" | "n" | "No" | "no" => BooleanInput::No,
//         _ => BooleanInput::Invalid,
//     }
// }

// impl BooleanInput {
//     pub fn is_affirmative(&self) -> bool {
//         match self {
//             BooleanInput::Yes => true,
//             BooleanInput::No => false,
//             BooleanInput::Invalid => false,
//         }
//     }
//
//     pub fn is_valid(&self) -> bool {
//         match self {
//             BooleanInput::Yes => true,
//             BooleanInput::No => true,
//             BooleanInput::Invalid => false,
//         }
//     }
//
//     pub fn invalid_reason(&self) -> &'static str {
//         match self {
//             BooleanInput::Invalid => "Input received must be 'yes' or 'no'",
//             _ => panic!("Invalid reason requested for valid `Answer`"),
//         }
//     }
// }

// impl ToString for BooleanInput {
//     fn to_string(&self) -> String {
//         match self {
//             BooleanInput::Yes => "yes",
//             BooleanInput::No => "no",
//             BooleanInput::Invalid => "invalid",
//         }
//         .parse()
//         .unwrap()
//     }
// }
