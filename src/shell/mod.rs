mod fish;

use std::{env, str::FromStr};

use anyhow::{bail, Context, Result};
use Shell::*;

#[derive(Debug)]
pub enum Shell {
    Fish,
}

impl FromStr for Shell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "fish" => Ok(Fish),
            _ => bail!("Shell is not yet supported: {}", s),
        }
    }
}

impl Shell {
    fn alias(&self, aliases: &[String]) -> Result<()> {
        match self {
            Fish => fish::alias(aliases),
        }
    }

    fn unalias(&self, aliases: &[String]) -> Result<()> {
        match self {
            Fish => fish::unalias(aliases),
        }
    }
}

pub fn alias(aliases: &[String]) -> Result<()> {
    get_shell()?.alias(aliases)
}

pub fn unalias(aliases: &[String]) -> Result<()> {
    get_shell()?.unalias(aliases)
}

fn get_shell() -> Result<Shell> {
    let shell_path =
        env::var("SHELL").context("Could not determine current shell. Is SHELL set?")?;

    let (_, shell) = shell_path
        .rsplit_once("/")
        .with_context(|| format!("Failed to get shell from SHELL path: {}", shell_path))?;

    str::parse::<Shell>(shell)
}
