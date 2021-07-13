use serde_derive::Deserialize;
use std::{borrow::Borrow, collections::{BTreeMap, HashSet}, ops::Deref, path::PathBuf};

use super::trigger::Trigger;

#[derive(Debug, Deserialize)]
pub struct Wrapper {
    pub description: String,
    pub triggers: Vec<Trigger>,
    pub arguments: Vec<String>,
}

pub struct SimpleWrapper {
    pub description: String,
    pub trigger: Trigger,
    pub arguments: Vec<String>,
}

impl Wrapper {
    /// Returns the longest matching trigger
    pub fn longest_match<'iter> (&self, args: impl IntoIterator<Item = impl AsRef<str>>) -> Option<&Trigger> {
        self.triggers
            .iter()
            .filter(|trigger| trigger.matches(args))
            .max()
    }
}

// pub fn get_command<'iter> (trigger: &Trigger, args: impl IntoIterator<Item = &'iter str>) {
//     args.into_iter()
//         .filter(|arg| trigger.con)
// }