mod linkfile;
mod linker;
mod stats;
mod error;

use std::path;
use clap::{Parser, ValueEnum};

use crate::linkfile::Linkfile;
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
    linkfile: path::PathBuf,

    #[arg(
        value_enum,
        short = 'm',
        long = "mode",
        default_value_t = Mode::Strict
    )]
    mode: Mode,
}

fn report_stats(stats: &Stats) {
    println!("Stats: {}", stats);
}

fn perform_linking(args: &Cli) -> Result<(), Error> {
    let linkfile_path = &args.linkfile.canonicalize()?;
    let linkfile_dir = linkfile_path.parent()
        .ok_or_else(
            || Error::Misc("Cannot find linkfile root"))?;

    let content = std::fs::read_to_string(&args.linkfile)?;
    let linkfile: Linkfile = toml::from_str(&content)?;

    let stats = linker::ensure_links(args.mode, &linkfile_dir, &linkfile)?;

    report_stats(&stats);

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    perform_linking(&args)
}
