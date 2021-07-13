use crate::wrapper;

use config::{
    ConfigError,
    File,
    Environment,
};
use serde_derive::Deserialize;
use std::{collections::BTreeMap, ops::Deref, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub aliases: Vec<Alias>,
}