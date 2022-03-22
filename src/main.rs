mod wrap;
mod shell;

use anyhow::{Context, Result};
use clap::{AppSettings, Subcommand, Parser};
use std::path::PathBuf;
use wrap::Config;

#[derive(Parser)]
#[clap(
    about,
    author,
    setting(AppSettings::InferSubcommands),
    setting(AppSettings::SubcommandRequiredElseHelp),
    setting(AppSettings::TrailingVarArg),
    version,
)]
struct Args {
    /// The config files to read from
    ///
    /// If specified, the default config files will not be used,
    /// and instead these config files will be merged with the first file
    /// having the least precedence
    #[clap(short, long)]
    config: Vec<PathBuf>,

    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Install aliases for the current shell
    ///
    /// Currently supported shells: [fish]
    Install,

    /// Uninstall aliases for the current shell
    Uninstall,

    /// Execute an alias
    #[clap(
        // So that trailing --help/--dry-run pass-through to the alias
        setting(AppSettings::TrailingVarArg),
    )]
    Run {
        /// Print the command instead of running it
        #[clap(short = 'n', long)]
        dry_run: bool,

        /// Alias to run
        alias: String,

        /// Positional arguments to pass to the underlying command
        #[clap(last = true, takes_value = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

const DRY_RUN_GLOBAL: &str = "--dry-run";

fn main() -> Result<()> {
    color_backtrace::install();
    pretty_env_logger::init();

    let args = Args::parse();

    let mut config = Config::new(args.config.iter())?;

    match args.action {
        Action::Install => {
            let aliases = config.aliases.get_aliases();
            shell::alias(&aliases)?;
            println!("Shell aliases installed. You may need to restart your shell session to take effect");
        },

        Action::Uninstall => {
            let aliases = config.aliases.get_aliases();
            shell::unalias(&aliases)?;
            println!("Shell aliases uninstalled. You may need to restart your shell session to take effect");
        },

        Action::Run { dry_run, alias, mut args, } => {
            // Determine if this is a dry run before we mutate the arguments
            let dry_run = global_dry_run(&mut args) || dry_run;

            let alias = config.aliases.get_alias(&alias).context("No matching alias found")?;
            let command = alias.get_command(args, &mut config.variables)?;

            if dry_run {
                println!("{}", command);
            } else {
                let mut command: exec::Command = command.into();
                return Err(command.exec().into());
            }
        },
    }

    Ok(())
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
