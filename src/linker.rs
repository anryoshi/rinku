use std::{env, fs, io, os, path};

use std::path::{Path, PathBuf};

use crate::dotfiles::{Destination, Dotfiles, Environment, Link, Target};
use crate::Mode;

pub use crate::error::Error;
pub use crate::stats::Stats;

extern crate dirs;

#[derive(Debug)]
pub enum OperationState {
    Doable,
    MissingSource,
    DestExist,
}

/// Resolved absolute pathes to be linked
#[derive(Debug)]
pub struct Operation {
    source: PathBuf,
    dest: PathBuf,
    state: OperationState,
}

impl Operation {
    fn determine_status(source: &PathBuf, dest: &PathBuf) -> OperationState {
        OperationState::Doable
    }

    fn new(source: PathBuf, dest: PathBuf) -> Operation {
        let state = Operation::determine_status(&source, &dest);
        Operation {
            source: source,
            dest: dest,
            state: state,
        }
    }

    fn source(&self) -> &PathBuf {
        &self.source
    }

    fn dest(&self) -> &PathBuf {
        &self.dest
    }

    fn link(self) {}
}

pub struct Linker {
    pub ops: Vec<Operation>,
}

impl Linker {
    fn unwrap_link(env: &Environment, root: &Path, link: &Link) -> Vec<Result<Operation, Error>> {
        // TODO: Check that this is relative path and not absolute
        let source = Path::new(&link.source);
        let source = root.join(source);

        let to_op = |dest| {
            let dest = Path::new(&dest);
            Operation::new(source.to_path_buf(), dest.to_path_buf())
        };

        let dest = match &link.target {
            Target::Unified(dest) => Some(dest),
            Target::Platform(platforms) => {
                if let Some(dest) = platforms.get(env) {
                    Some(dest)
                } else {
                    None
                }
            }
        };

        match &dest {
            Some(Destination::Single(dest)) => {
                vec![Ok(to_op(dest))]
            }
            Some(Destination::Multi(dests)) => dests.into_iter().map(|x| Ok(to_op(x))).collect(),
            None => Vec::new(),
        }
    }

    fn unwrap_dotfiles(
        root: &path::Path,
        dotfiles: &Dotfiles,
    ) -> Result<Vec<Operation>, Vec<Error>> {
        let environment = os_to_environment(env::consts::OS).expect("unknown os");

        let (operations, errors): (Vec<_>, Vec<_>) = dotfiles
            .links
            .iter()
            .flat_map(|link| Linker::unwrap_link(&environment, root, link))
            .partition(Result::is_ok);

        if errors.is_empty() {
            Ok(operations.into_iter().map(Result::unwrap).collect())
        } else {
            Err(errors.into_iter().map(Result::unwrap_err).collect())
        }
    }

    pub fn new(root: &path::Path, dotfiles: &Dotfiles) -> Result<Linker, Vec<Error>> {
        let ops = Linker::unwrap_dotfiles(root, dotfiles)?;
        Ok(Linker { ops: ops })
    }

    pub fn link(self) {

    }
}

pub fn link_dotfiles(mode: Mode, root: &path::Path, dotfiles: &Dotfiles) -> Result<Stats, Error> {
    let environment = os_to_environment(env::consts::OS);
    match environment {
        Some(env) => link_in_environment(mode, root, dotfiles, env),
        None => Err(Error::Misc("Unknown environment")),
    }
}

fn link_in_environment(
    mode: Mode,
    root: &path::Path,
    dotfiles: &Dotfiles,
    environment: Environment,
) -> Result<Stats, Error> {
    let mut accumulated_stats = Stats::new();

    for link in dotfiles.links.iter() {
        let source = path::Path::new(&link.source);
        let source = root.join(source);
        let stats = match &link.target {
            Target::Unified(dest) => link_to_destination(mode, &source, &dest),
            Target::Platform(platforms) => {
                if let Some(d) = platforms.get(&environment) {
                    link_to_destination(mode, &source, &d)
                } else {
                    Ok(Stats::new())
                }
            }
        }?;
        accumulated_stats.aggregate(&stats);
    }

    return Ok(accumulated_stats);
}

fn link_to_destination(
    mode: Mode,
    source: &path::PathBuf,
    destination: &Destination,
) -> Result<Stats, Error> {
    fs::metadata(source)?;

    let mut stats = Stats::new();

    let mut links_counter = stats.new_item();
    match destination {
        Destination::Single(dest) => {
            let dest = path::Path::new(dest);
            if perform_link(mode, source, dest)? {
                links_counter();
            }
        }
        Destination::Multi(dests) => {
            for dest in dests {
                let dest = path::Path::new(dest);
                if perform_link(mode, source, dest)? {
                    links_counter();
                }
            }
        }
    }
    drop(links_counter);

    Ok(stats)
}

fn perform_link(mode: Mode, source: &path::Path, dest: &path::Path) -> Result<bool, Error> {
    match mode {
        Mode::Strict => {
            if fs::metadata(dest).is_ok() {
                eprintln!("WARN: file {} already exists in system", dest.display());
                return Ok(false);
            }
            let new_dest = expand_dest(dest);
            create_parent(&new_dest)?;
            println!("Linking {:?}, {:?}", source, &new_dest);
            platform_link(source, &new_dest)?
        }
        _ => {}
    }

    Ok(true)
}

fn os_to_environment(os: &str) -> Option<Environment> {
    match os {
        "linux" => Some(Environment::UnixLike),
        "macos" => Some(Environment::UnixLike),
        "windows" => Some(Environment::Windows),
        _ => None,
    }
}

fn expand_dest(dest: &path::Path) -> path::PathBuf {
    if dest.starts_with("~/") {
        let home_dir = dirs::home_dir().unwrap();

        let home_dir_components: Vec<_> = home_dir.components().collect();
        let start_components = &home_dir_components[..];

        let dest_components: Vec<_> = dest.components().collect();
        let end_components = &dest_components[1..];

        let new_path_buf: path::PathBuf =
            [start_components, end_components].concat().iter().collect();

        new_path_buf
    } else {
        dest.to_path_buf()
    }
}

fn create_parent(dest: &path::Path) -> io::Result<()> {
    let target_dir = dest.parent().unwrap();

    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?
    } else if !target_dir.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
    }

    Ok(())
}

#[cfg(target_family = "unix")]
fn platform_link(source: &path::Path, dest: &path::Path) -> io::Result<()> {
    os::unix::fs::symlink(source, dest)
}

#[cfg(target_family = "windows")]
fn platform_link(source: &path::Path, dest: &path::Path) -> io::Result<()> {
    os::windows::fs::symlink_file(source, dest)
}
