mod cli;
mod error;
mod linker;
mod linkfile;
mod printer;

use crate::error::Error;
use crate::linkfile::Linkfile;

use cli::{Cli, Parser};

fn save_zelda(args: &Cli) -> Result<(), Error> {
    let linkfile_path = &args.linkfile.canonicalize()
        .map_err(|e| Error::BadLinkfile(e))?;
    let linkfile_dir = linkfile_path
        .parent()
        .ok_or_else(|| Error::BadLinkfilePath)?;

    let content = std::fs::read_to_string(&args.linkfile)
        .map_err(|e| Error::BadLinkfile(e))?;

    let linkfile: Linkfile = toml::from_str(&content)?;

    let result = linker::do_linkage(args.mode, &linkfile_dir, &linkfile, &args.tags)?;

    printer::present_result(&result);

    Ok(())
}

fn main() {
    let args = Cli::parse();
    if let Err(err) = save_zelda(&args) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
