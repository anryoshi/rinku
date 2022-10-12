mod dotfiles;
mod linker;
mod stats;
mod error;

use std::path;
use clap::{Parser, ValueEnum};

use crate::dotfiles::Dotfiles;
use crate::stats::Stats;
use crate::error::Error;

#[derive(ValueEnum, Clone, Copy)]
pub enum Mode {
    /// Fails on the first collision error
    Strict,

    /// Only prints what will be override
    Dry,

    /// Override all existing links
    Force,
}

#[derive(Parser)]
struct Cli {
    dotfile: path::PathBuf,

    #[arg(
        value_enum,
        short = 'm',
        long = "mode",
        // TODO: Fina a better way to pass default value
        default_value = "strict"
    )]
    mode: Mode,
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
