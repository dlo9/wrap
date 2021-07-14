mod wrap;

use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use log::{debug, info};
use wrap::Config;

#[derive(structopt::StructOpt)]
#[structopt(
    rename_all = "kebab",
    rename_all_env = "screaming-snake",
    setting = structopt::clap::AppSettings::ColoredHelp,
    setting = structopt::clap::AppSettings::TrailingVarArg,
)]
struct Args {
    /// The file to publish to rabbit
    #[structopt(env, short, long)]
    config: Option<PathBuf>,

    /// Print the command to be run
    #[structopt(short = "n", long)]
    dry_run: bool,

    /// Positional arguments to pass to the underlying command
    args: Vec<String>,
}

const DRY_RUN_GLOBAL: &str = "--dry-run";

#[paw::main]
fn main(mut args: Args) -> Result<()> {
    color_backtrace::install();
    pretty_env_logger::init();

    let config = Config::new(&args.config)?;
    if args.args.is_empty() {
        bail!("No alias given");
    }

    // Determine if this is a dry run before we mutate the arguments
    let dry_run = global_dry_run(&mut args.args) || args.dry_run;

    let user_alias = args.args.remove(0);
    let alias = config.aliases
        .into_iter()
        .find(|alias| alias == &user_alias)
        .context("No matching alias found")?;
    let command = alias.get_command(args.args);

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
