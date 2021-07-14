use serde_derive::Deserialize;
use std::cmp::PartialEq;
use super::{command::Command, default_argument::DefaultArguments, keyword::Keywords};

#[derive(Debug, Deserialize)]
pub struct Alias {
    pub alias: String,
    pub program: String,
    #[serde(rename = "arguments")]
    pub default_arguments: DefaultArguments,
    pub keywords: Keywords,
}

impl<S> PartialEq<S> for Alias
where S: AsRef<str> {
    fn eq(&self, other: &S) -> bool {
        self.alias == other.as_ref()
    }
}

impl Alias {
    pub fn get_command(self, mut arguments: Vec<String>) -> Command {
        self.keywords.replace(&mut arguments);
        self.default_arguments.apply(&mut arguments);

        Command {
            program: self.program,
            arguments,
        }
    }
}
