mod fish;

use std::env;

use anyhow::{Context, Result};
use Shell::*;

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum Shell {
        // TODO: use correct case here
        fish,
    }
}

impl Shell {
    fn alias(&self, aliases: &[String]) -> Result<()> {
        match self {
            fish => fish::alias(aliases),
        }
    }

    fn unalias(&self, aliases: &[String]) -> Result<()> {
        match self {
            fish => fish::unalias(aliases),
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
    let shell_path = env::var("SHELL")
        .context("Could not determine current shell. Is SHELL set?")?;

    let (_, shell) = shell_path
        .rsplit_once("/")
        .with_context(|| format!("Failed to get shell from SHELL path: {}", shell_path))?;

    str::parse::<Shell>(shell)
        .with_context(|| format!("Shell is not yet supported: {}", shell))
}
