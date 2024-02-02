use serde_derive::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
struct Keyword {
    pub keys: HashSet<String>,
    pub values: Vec<String>,
}

impl Keyword {
    // TODO: eventually use `impl Iterator<Item = impl AsRef<str>>`
    // This requires either a custom iterator impl with the GAT feature,
    // or the ability to return different impls from different if branches
    fn replace(self, to_replace: &mut Vec<String>) {
        if let Some(key_index) = to_replace
            .iter()
            .position(|possible_key| self.keys.contains(possible_key))
        {
            // Remove the found key
            to_replace.remove(key_index);

            // Insert the replacement values at the key location
            for value in self.values.into_iter().rev() {
                to_replace.insert(key_index, value);
            }
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Keywords(Vec<Keyword>);

impl Keywords {
    pub fn replace(self, to_replace: &mut Vec<String>) {
        for keyword in self.0.into_iter() {
            keyword.replace(to_replace);
        }
    }
}
