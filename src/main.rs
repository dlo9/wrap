mod config;

use anyhow::{bail, Result};
use std::path::PathBuf;
use log::info;
use crate::config::Config;

#[derive(structopt::StructOpt)]
#[structopt(rename_all = "kebab", rename_all_env = "screaming-snake", setting = structopt::clap::AppSettings::ColoredHelp)]
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

#[paw::main]
fn main(args: Args) -> Result<()> {
    color_backtrace::install();
    pretty_env_logger::init();

    let config = Config::new(args.config)?;
    if args.args.is_empty() {
        bail!("No positional arguments given");
    }

    for (description, wrapper) in config.wrappers {
        if args.args.starts_with(&wrapper.trigger) {
            info!("Trigger detected for '{}'", description);

            // Remove the trigger to get the remaining positional arguments
            let append_args = &args.args[wrapper.trigger.len()..];
            //debug!("Trigger detected for '{}'", append_args);
            let command_args: Vec<&String> = wrapper.args.iter().chain(append_args).collect();

            if args.dry_run {
                let command_args = command_args.iter()
                    .map(|s| format!("\"{}\"", s))
                    .collect::<Vec<String>>()
                    .join(" ");

                println!("{} {}", wrapper.command, command_args);
                return Ok(())
            } else {
                let error = exec::Command::new(wrapper.command)
                    .args(&command_args)
                    .exec();

                return Err(error.into());

            }
        }
    }

    bail!("No trigger found: {:?}", args.args)
}
