mod dotfiles;
mod linker;
mod linker_stats;
mod linker_error;

use std::path;
use structopt::StructOpt;
use strum_macros::EnumString;

use crate::dotfiles::Dotfiles;
use crate::linker_error::LinkerError;

#[derive(Copy, Clone, EnumString)]
pub enum Mode {
    #[strum(serialize = "strict")]
    Strict
}

#[derive(StructOpt)]
struct Cli {
    dotfile: path::PathBuf,

    #[structopt(short = "m", long = "mode", default_value = "strict")]
    mode: Mode,
}

fn main() -> Result<(), LinkerError> {
    let args = Cli::from_args();

    let dotfile_path = &args.dotfile.canonicalize()
        .or_else(|e| Err(LinkerError::Io(e)))?;
    let dotfile_dir = dotfile_path.parent()
        .ok_or_else(
            || LinkerError::Misc("Cannot find dotfiles root".to_string()))?;

    let content = std::fs::read_to_string(&args.dotfile)
        .or_else(|e| Err(LinkerError::Io(e)))?;

    let dotfiles: Dotfiles = toml::from_str(&content)
        .or_else(|e| Err(LinkerError::Parse(e)))?;

    let stats = linker::link_dotfiles(args.mode, &dotfile_dir, &dotfiles)?;

    println!("[LinkerStats]{}", stats);

    Ok(())
}
