mod config;

use anyhow::{
    bail,
    Result
};
use std::path::PathBuf;
use log::info;
use crate::config::Config;

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
fn main(args: Args) -> Result<()> {
    color_backtrace::install();
    pretty_env_logger::init();

    let config = Config::new(&args.config)?;
    if args.args.is_empty() {
        bail!("No positional arguments given");
    }

    // Find the wrapper with the longest matching trigger
    let wrapper = config.wrappers.into_iter()
        .filter(|(_, wrapper)| args.args.starts_with(&wrapper.trigger))
        .inspect(|(description, _)| info!("Trigger matches wrapper '{}'", description))
        .max_by_key(|(_, wrapper)| wrapper.trigger.len());

    // Exit when no trigger is found
    let (description, wrapper) = if wrapper.is_some() {
        wrapper.unwrap()
    } else {
        bail!("No trigger found: {:?}", args.args);
    };

    info!("Executing maximally specific wrapper '{}'", description);

    // Remove the trigger to get the remaining positional arguments
    let mut dry_run = args.dry_run;
    let append_args = args.args[wrapper.trigger.len()..]
        .into_iter()
        .filter(|s|
            // Hacky way to allow the dry-run flag to be specified globally
            match s.as_str() {
                DRY_RUN_GLOBAL => {
                    dry_run = true;
                    false
                }
                _ => true
            }
        );

    //debug!("Trigger detected for '{}'", append_args);
    let command_args: Vec<&String> = wrapper.args.iter().chain(append_args).collect();

    if dry_run {
        let command_args = command_args.iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<String>>()
            .join(" ");

        println!("{} {}", wrapper.command, command_args);
        return Ok(())
    } else {
        let error = exec::Command::new(&wrapper.command)
            .args(&command_args)
            .exec();

        return Err(error.into());

    }
}
