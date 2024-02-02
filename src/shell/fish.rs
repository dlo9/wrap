use anyhow::{Context, Result};
use dirs::home_dir;
use std::fs::{remove_file, write};
use std::path::PathBuf;

pub fn alias(aliases: &[String]) -> Result<()> {
    let mut contents = String::new();
    for alias in aliases {
        contents.push_str(&format!("alias {0}=\"wrap {0}\"\n", alias));
    }

    let path = get_config_file_path()?;
    write(&path, contents).with_context(|| {
        format!(
            "Encountered error while removing file {}",
            path.to_string_lossy()
        )
    })
}

pub fn unalias(_aliases: &[String]) -> Result<()> {
    let path = get_config_file_path()?;
    remove_file(&path).with_context(|| {
        format!(
            "Encountered error while removing file {}",
            path.to_string_lossy()
        )
    })
}

fn get_config_file_path() -> Result<PathBuf> {
    let mut dir = home_dir().context("Could not get home directory")?;
    dir.push(".config/fish/conf.d/wrap.fish");
    Ok(dir)
}
