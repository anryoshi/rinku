mod cli;
mod error;
mod linker;
mod linkfile;
mod printer;

use crate::error::Error;
use crate::linkfile::Linkfile;

use cli::{Cli, Parser};

fn save_zelda(args: &Cli) -> Result<(), Error> {
    let linkfile_path = &args.linkfile.canonicalize()?;
    let linkfile_dir = linkfile_path
        .parent()
        .ok_or_else(|| Error::BadLinkfilePath)?;

    let content = std::fs::read_to_string(&args.linkfile)?;
    let linkfile: Linkfile = toml::from_str(&content)?;

    let result = linker::do_linkage(args.mode, &linkfile_dir, &linkfile)?;

    printer::present_result(&result);

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    save_zelda(&args)
}
