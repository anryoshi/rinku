mod dotfiles;
mod linker;
mod stats;
mod error;

use std::path;
use clap::{Parser, ArgEnum};
use clap::{crate_version};

use crate::dotfiles::Dotfiles;
use crate::stats::Stats;
use crate::error::Error;

#[derive(ArgEnum, Clone, Copy)]
pub enum Mode {
    /// Only prints what will be done
    Dry,

    /// Check that operation could be done without overrides
    /// and perform it only then
    Strict,

    /// Override all existing links during the process
    Force,
}

#[derive(Parser)]
#[clap(
    name = "rinku",
    about = "simple link automation tool",
    version = crate_version!()
)]
struct Cli {
    #[clap(
        about = "Path to dotfile.toml"
    )]
    dotfile: path::PathBuf,

    #[clap(
        arg_enum,
        short = 'm',
        long = "mode",
        // TODO: Find a better way to pass default value
        default_value = "dry",
        about = "Execution mode"
    )]
    mode: Mode,

    #[clap(
        short = 'r',
        long = "root",
        about = "Linker tree root"
    )]
    root: Option<String>,
}

fn report_stats(stats: &Stats) {
    println!("Stats: {}", stats);
}

fn perform_linking(args: &Cli) -> Result<(), Error> {
    let dotfile_path = &args.dotfile.canonicalize()?;
    let dotfile_dir = dotfile_path.parent()
        .ok_or_else(
            || Error::Misc("Cannot find dotfiles root"))?;

    let content = std::fs::read_to_string(&args.dotfile)?;
    let dotfiles: Dotfiles = toml::from_str(&content)?;

    let stats = linker::link_dotfiles(args.mode, &dotfile_dir, &dotfiles)?;

    report_stats(&stats);

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    perform_linking(&args)
}
