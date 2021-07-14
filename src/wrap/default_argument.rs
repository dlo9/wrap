use std::collections::HashSet;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct DefaultArgument {
    pub key: String,
    pub value: String,
    pub cleared_by: HashSet<String>,
}

impl DefaultArgument {
    // Potentially apply the default argument to the user-provided runtime arguments.
    // If a `cleared_by` argument exists in the list, no change will be made.
    // Otherwise, the default argument will be appended to the list.
    fn apply(self, arguments: &mut Vec<String>) {
        if arguments.iter().all(|clearing_key| !self.cleared_by.contains(clearing_key)) {
            // No clearing key found, add the key/value
            arguments.push(self.key);
            arguments.push(self.value);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DefaultArguments(Vec<DefaultArgument>);

impl DefaultArguments {
    pub fn apply(self, arguments: &mut Vec<String>) {
        for default_argument in self.0.into_iter() {
            default_argument.apply(arguments);
        }
    }
}
