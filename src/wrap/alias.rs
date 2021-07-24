use anyhow::Result;
use serde_derive::Deserialize;
use std::cmp::PartialEq;
use super::{
    command::Command,
    default_argument::DefaultArguments,
    keyword::Keywords,
    variable::Variables,
};

#[derive(Debug, Deserialize)]
pub struct Alias {
    pub alias: String,
    pub program: String,
    #[serde(rename = "arguments", default)]
    pub default_arguments: DefaultArguments,
    #[serde(default)]
    pub keywords: Keywords,
}

impl<S> PartialEq<S> for Alias
where S: AsRef<str> {
    fn eq(&self, other: &S) -> bool {
        self.alias == other.as_ref()
    }
}

impl Alias {
    pub fn get_command(self, mut arguments: Vec<String>, variables: &mut Variables) -> Result<Command> {
        self.keywords.replace(&mut arguments);
        self.default_arguments.apply(&mut arguments);
        let arguments = variables.apply(&arguments)?;

        Ok(Command {
            program: self.program,
            arguments,
        })
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Aliases(Vec<Alias>);

impl Aliases {
    pub fn get_alias(self, alias: &str) -> Option<Alias> {
        self.0
            .into_iter()
            .find(|a| a == &alias)
    }

    pub fn get_aliases(self) -> Vec<String> {
        self.0
            .into_iter()
            .map(|alias| alias.alias)
            .collect()
    }
}
