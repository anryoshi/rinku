pub use crate::error::Error;

use std::cmp::Ordering;
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs, io, os, path};

use crate::cli::Mode;
use crate::linkfile::*;

#[derive(Debug)]
pub enum TargetState {
    Absent,
    AlienNode(Metadata),
    AlienLink(Metadata),
    Linked(Metadata),
}

#[derive(Debug)]
pub struct LinkTask {
    pub source: path::PathBuf,
    pub source_metadata: fs::Metadata,
    pub target: path::PathBuf,
    pub target_state: TargetState,
}

#[derive(Debug)]
pub enum LinkResult {
    Existed,
    Skipped,
    Success,
    IoError(io::Error),
}

#[derive(Debug)]
pub struct LinkState {
    pub task: LinkTask,
    pub result: LinkResult,
}

#[derive(Debug)]
pub enum LinkageResult {
    DryResult(Vec<LinkTask>),
    PreconditionFailed(&'static str),
    Completed(Vec<LinkState>),
}

pub fn do_linkage(
    mode: Mode,
    root: &path::Path,
    linkfile: &Linkfile,
) -> Result<LinkageResult, Error> {
    let environment = Environment::from_str(env::consts::FAMILY)?;

    let mut link_tasks = aggregate_link_tasks(environment, &root, &linkfile.links)?;
    link_tasks.sort_by(compare_link_tasks);

    Ok(match mode {
        Mode::Dry => dry_link_tasks(link_tasks),
        Mode::Strict => link_strictly(link_tasks),
        Mode::Lazy => link_lazy(link_tasks),
        Mode::Force => link_forcefully(link_tasks),
    })
}

fn compare_link_tasks(l: &LinkTask, r: &LinkTask) -> Ordering {
    // Waiting for Discriminant with Ord trait implemented
    // (╯°□°）╯︵ ┻━┻

    let l_target_state = &l.target_state;
    let r_target_state = &r.target_state;

    let l_discriminant = std::mem::discriminant(l_target_state);
    let r_discriminant = std::mem::discriminant(r_target_state);

    if l_discriminant == r_discriminant {
        Ordering::Equal
    } else {
        match l_target_state {
            TargetState::Absent => Ordering::Greater,
            TargetState::AlienNode(_) => match r_target_state {
                TargetState::Absent => Ordering::Less,
                _ => Ordering::Greater,
            },
            TargetState::AlienLink(_) => match r_target_state {
                TargetState::Linked(_) => Ordering::Greater,
                _ => Ordering::Less,
            },
            TargetState::Linked(_) => Ordering::Less,
        }
    }
}

fn aggregate_link_tasks(
    environment: Environment,
    root: &path::Path,
    links: &Vec<Link>,
) -> Result<Vec<LinkTask>, io::Error> {
    let result: Result<Vec<Vec<LinkTask>>, io::Error> = links
        .iter()
        .map(|link| create_link_tasks(environment, root, link))
        .collect();

    result.map(|tasks| tasks.into_iter().flatten().collect())
}

fn create_link_tasks(
    environment: Environment,
    root: &path::Path,
    link: &Link,
) -> Result<Vec<LinkTask>, io::Error> {
    let source = root.join(path::Path::new(&link.source));
    let source_metadata = fs::metadata(&source)?;

    let destination = match &link.target {
        Target::Unified(destination) => destination,
        Target::Platform(platforms) => {
            if let Some(destination) = platforms.get(&environment) {
                destination
            } else {
                // Current OS is not supported
                return Ok(vec![]);
            }
        }
    };

    let targets = match destination {
        Destination::Single(target) => vec![path::Path::new(target).to_path_buf()],
        Destination::Multi(targets) => targets
            .iter()
            .map(|target| path::Path::new(target).to_path_buf())
            .collect(),
    };

    targets
        .into_iter()
        .map(|target| {
            let target = expand_dest(&target);
            let target_state = examine_target_state(&target, &source)?;
            Ok(LinkTask {
                source: source.clone(),
                source_metadata: source_metadata.clone(),
                target: target,
                target_state: target_state,
            })
        })
        .collect()
}

fn examine_target_state(target: &path::Path, source: &path::Path) -> io::Result<TargetState> {
    assert!(target.is_absolute());
    assert!(source.is_absolute());

    let target_exists = target.try_exists()?;

    if !target_exists {
        return Ok(TargetState::Absent);
    }

    if !target.is_symlink() {
        return Ok(TargetState::AlienNode(target.metadata()?));
    }

    let target_metadata = target.symlink_metadata()?;
    let target_destination = target.read_link()?;
    if target_destination != source {
        return Ok(TargetState::AlienLink(target_metadata));
    }

    return Ok(TargetState::Linked(target_metadata));
}

fn dry_link_tasks(linktasks: Vec<LinkTask>) -> LinkageResult {
    LinkageResult::DryResult(linktasks)
}

fn is_link_task_target_absent(link_task: &LinkTask) -> bool {
    if let TargetState::Absent = link_task.target_state {
        true
    } else {
        false
    }
}

fn link_strictly(link_tasks: Vec<LinkTask>) -> LinkageResult {
    if !link_tasks.iter().all(is_link_task_target_absent) {
        return LinkageResult::PreconditionFailed("Some of the targets exists");
    }

    let result = link_tasks
        .into_iter()
        .map(|link_task| LinkState {
            result: execute_linktask(&link_task, false),
            task: link_task,
        })
        .collect();

    LinkageResult::Completed(result)
}

fn link_lazy(link_tasks: Vec<LinkTask>) -> LinkageResult {
    let result = link_tasks
        .into_iter()
        .filter(is_link_task_target_absent)
        .map(|link_task| LinkState {
            result: execute_linktask(&link_task, false),
            task: link_task,
        })
        .collect();

    LinkageResult::Completed(result)
}

fn link_forcefully(link_tasks: Vec<LinkTask>) -> LinkageResult {
    let result = link_tasks
        .into_iter()
        .map(|link_task| LinkState {
            result: execute_linktask(&link_task, true),
            task: link_task,
        })
        .collect();

    LinkageResult::Completed(result)
}

fn link_without_overriding(source: &Path, target: &Path) -> LinkResult {
    assert!(source.exists());
    assert!(!target.exists());

    if let Err(err) = platform_link(&source, &target) {
        LinkResult::IoError(err)
    } else {
        LinkResult::Success
    }
}

fn link_with_overriding(source: &Path, target: &Path) -> LinkResult {
    assert!(source.exists());
    assert!(target.exists());

    if let Err(err) = backup_target(&target) {
        LinkResult::IoError(err)
    } else {
        link_without_overriding(&source, &target)
    }
}

fn backup_target(target: &Path) -> io::Result<()> {
    assert!(target.exists());

    let new_target_name = find_free_backup_target_name(target).ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Cannot find suitable backup name",
    ))?;

    fs::rename(target, new_target_name)
}

fn find_free_backup_target_name(target: &Path) -> Option<PathBuf> {
    let target_components: Vec<_> = target.components().collect();
    let (last, elements) = target_components.split_last()?;

    let name_constructor = |i| {
        let mut new_end = std::ffi::OsString::new();
        new_end.push(last);
        new_end.push(format!(".bak.{}", i));

        let new_component = path::Component::Normal(&new_end);

        let new_target: PathBuf = [elements, std::slice::from_ref(&new_component)]
            .concat()
            .iter()
            .collect();

        new_target
    };

    for i in 1..100 {
        let new_target_name = name_constructor(i);

        if !new_target_name.exists() {
            return Some(new_target_name);
        }
    }

    None
}

fn execute_linktask(link_task: &LinkTask, overwrite: bool) -> LinkResult {
    match link_task.target_state {
        TargetState::Absent => link_without_overriding(&link_task.source, &link_task.target),
        TargetState::AlienNode(_) => {
            if overwrite {
                link_with_overriding(&link_task.source, &link_task.target)
            } else {
                LinkResult::Skipped
            }
        }
        TargetState::AlienLink(_) => {
            if overwrite {
                link_with_overriding(&link_task.source, &link_task.target)
            } else {
                LinkResult::Skipped
            }
        }
        TargetState::Linked(_) => LinkResult::Existed,
    }
}

fn expand_dest(dest: &Path) -> PathBuf {
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

fn create_parent(dest: &Path) -> io::Result<()> {
    let target_dir = dest.parent().unwrap();

    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?
    } else if !target_dir.is_dir() {
        // TODO: Replace error type
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Parent path for the target is not directory!",
        ));
    }

    Ok(())
}

#[cfg(target_family = "unix")]
fn platform_link(source: &Path, dest: &Path) -> io::Result<()> {
    create_parent(dest)?;
    os::unix::fs::symlink(source, dest)
}

#[cfg(target_family = "windows")]
fn platform_link(source: &Path, dest: &Path) -> io::Result<()> {
    create_parent(dest)?;

    if source.is_dir {
        os::windows::fs::symlink_dir(source, dest)
    } else {
        os::windows::fs::symlink_file(source, dest)
    }
}
