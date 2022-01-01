mod wrap;
mod shell;

use anyhow::{Context, Result, bail};
use clap::{AppSettings, Parser};
use std::path::PathBuf;
use wrap::Config;

#[derive(Parser)]
#[clap(
    about,
    author,
    //setting(AppSettings::TrailingVarArg),
    version,
)]
struct Args {
    /// Install aliases for the current shell
    ///
    /// Currently supported shells: [fish]
    #[clap(long)]
    alias: bool,

    /// The config files to read from
    ///
    /// If specified, the default config files will not be used,
    /// and instead these config files will be merged with the first file
    /// having the least precedence
    #[clap(short, long)]
    config: Vec<PathBuf>,

    /// Print the command to be run
    #[clap(short = 'n', long, conflicts_with_all = &["alias", "unalias"])]
    dry_run: bool,

    /// Uninstall aliases for the current shell
    #[clap(long, conflicts_with = "alias")]
    unalias: bool,

    /// Positional arguments to pass to the underlying command
    args: Vec<String>,
}

const DRY_RUN_GLOBAL: &str = "--dry-run";

fn main() -> Result<()> {
    color_backtrace::install();
    pretty_env_logger::init();

    let mut args = Args::parse();

    let mut config = Config::new(args.config.iter())?;

    if args.alias {
        let aliases = config.aliases.get_aliases();
        shell::alias(&aliases)?;
        println!("Shell aliases installed. You may need to restart your shell session to take effect");
        return Ok(());
    }

    if args.unalias {
        let aliases = config.aliases.get_aliases();
        shell::unalias(&aliases)?;
        println!("Shell aliases uninstalled. You may need to restart your shell session to take effect");
        return Ok(());
    }

    if args.args.is_empty() {
        bail!("No alias given");
    }

    // Determine if this is a dry run before we mutate the arguments
    let dry_run = global_dry_run(&mut args.args) || args.dry_run;

    let user_alias = args.args.remove(0);
    let alias = config.aliases.get_alias(&user_alias).context("No matching alias found")?;
    let command = alias.get_command(args.args, &mut config.variables)?;

    if dry_run {
        println!("{}", command);
        Ok(())
    } else {
        let mut command: exec::Command = command.into();
        Err(command.exec().into())
    }
}

// Returns true if the global dry-run flag was found in the arguments,
// and then removes the flag so that it's no printed in the dry-run
fn global_dry_run(arguments: &mut Vec<String>) -> bool {
    if let Some(index) = arguments.iter().position(|argument| argument == DRY_RUN_GLOBAL) {
        arguments.remove(index);
        true
    } else {
        false
    }
}
