use std::{io, os, fs, env, path};

use crate::Mode;
use crate::dotfiles::{
    Dotfiles,
    Destination,
    Target,
    Environment
};

pub use crate::linker_stats::LinkerStats;
pub use crate::linker_error::LinkerError;

pub fn link_dotfiles(
    mode: Mode,
    root: &path::Path,
    dotfiles: &Dotfiles
) -> Result<LinkerStats, LinkerError> {
    let environment = os_to_environment(env::consts::OS);
    match environment {
        Some(env) => link_in_environment(mode, root, dotfiles, env),
        None => Err(LinkerError::Misc("Unknown environment".to_string()))
    }
}

fn link_in_environment(
    mode: Mode,
    root: &path::Path,
    dotfiles : &Dotfiles,
    environment : Environment
) -> Result<LinkerStats, LinkerError> {
    let mut accumulated_stats = LinkerStats::new();

    for link in dotfiles.links.iter() {
        let source = path::Path::new(&link.source);
        let source = root.join(source);
        let stats = match &link.target {
            Target::Unified(dest) => {
                link_to_destination(mode, &source, &dest)
            },
            Target::Platform(platforms) => {
                if let Some(d) = platforms.get(&environment.to_string()) {
                    link_to_destination(mode, &source, &d)
                } else {
                    Ok(LinkerStats::new())
                }
            }
        }?;
        accumulated_stats.aggregate(&stats); 
    }
    
    return Ok(accumulated_stats);
}

fn link_to_destination(
    mode : Mode,
    source : &path::PathBuf,
    destination : &Destination
) -> Result<LinkerStats, LinkerError> {
    fs::metadata(source)?;
    
    let mut stats = LinkerStats::new();

    let mut links_counter = stats.new_item();
    match destination {
        Destination::Single(dest) => {
            let dest = path::Path::new(dest);
            if perform_link(mode, source, dest)? {
                links_counter();
            }
        },
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

fn perform_link(
    mode: Mode,
    source: &path::Path,
    dest: &path::Path
) -> Result<bool, LinkerError> {
    match mode {
        Mode::Strict => {
            if fs::metadata(dest).is_ok() {
                eprintln!("WARN: file {} already exists in system", dest.display());
                return Ok(false)
            }
            platform_link(source, dest)?
        }
    }

    Ok(true)
}


fn os_to_environment(os: &str) -> Option<Environment> {
    match os {
        "linux" => Some(Environment::UnixLike),
        "macos" => Some(Environment::UnixLike),
        "windows" => Some(Environment::Windows),
        _ => None
    }
}

#[cfg(target_family = "unix")]
fn platform_link(source : &path::Path, dest : &path::Path) -> io::Result<()> {
    os::unix::fs::symlink(source, dest)
}

#[cfg(target_family = "windows")]
fn platform_link(source : &path::Path, dest : &path::Path) -> io::Result<()> {
    os::windows::fs::symlink_file(source, dest)
}
