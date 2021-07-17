use std::{
    collections::HashMap,
    env,
};
use serde_derive::Deserialize;

// #[derive(Debug, Deserialize)]
// struct Variable {
//     pub name: String,
//     pub value: String,
// }

// impl Variable {
//     // Potentially apply the default argument to the user-provided runtime arguments.
//     // If a `cleared_by` argument exists in the list, no change will be made.
//     // Otherwise, the default argument will be appended to the list.
//     fn apply(self, arguments: &mut Vec<String>) {
//         arguments.iter_mut()
//         if arguments.iter().all(|clearing_key| !self.cleared_by.contains(clearing_key)) {
//             // No clearing key found, add the key/value
//             arguments.push(self.key);

//             if let Some(value) = self.value {
//                 arguments.push(value);
//             }
//         }
//     }
// }

#[derive(Debug, Default, Deserialize)]
pub struct Variables(HashMap<String, String>);
//type DefaultArguments = Vec<DefaultArgument>;

// impl Default for Variables {
// }

impl Variables {
    pub fn apply(&self, arguments: &mut Vec<String>) {
        for argument in arguments {
            if let Some(variable_value) = self.0.get(argument) {
                // Must clone the string since there could be multiple of the same variable in arguments
                *argument = variable_value.to_string();
            }
        }
    }
}
