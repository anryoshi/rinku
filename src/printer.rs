use crate::linker::{LinkResult, LinkState, LinkTask, LinkageResult, TargetState};
use colored::*;

pub fn present_result(linkage_result: &LinkageResult) {
    match linkage_result {
        LinkageResult::DryResult(link_tasks) => present_dry_result(&link_tasks),
        LinkageResult::PreconditionFailed(reason) => {
            println!(
                "Precondition failed: {}. Try to run in the dry mode.",
                reason
            );
        }
        LinkageResult::Completed(link_states) => present_completed(&link_states),
    }
}

fn present_dry_result(link_tasks: &Vec<LinkTask>) {
    let arrow = "->".magenta().bold();
    let to_link = "TODO".yellow().bold();
    let alien_file = "ALIEN".red().bold();
    let alien_link = "ALIEN".red().bold();
    let linked = "LINKED".green().bold();

    let print_status = |status, source: &str, target: &str| {
        println!("{: <6} :: {} {} {}", &status, &source, &arrow, &target);
    };

    link_tasks.iter().for_each(|link_task| {
        let source = &link_task.source.display().to_string();
        let target = &link_task.target.display().to_string();

        match &link_task.target_state {
            TargetState::Absent => {
                print_status(&to_link, &source, &target);
            }
            TargetState::AlienNode(_metadata) => {
                print_status(&alien_file, &source, &target);
            }
            TargetState::AlienLink(_metadata) => {
                print_status(&alien_link, &source, &target);
            }
            TargetState::Linked(_metadata) => {
                print_status(&linked, &source, &target);
            }
        }
    })
}

fn present_completed(link_states: &Vec<LinkState>) {
    let arrow = "->".magenta().bold();
    let error = "ERROR".red().bold();
    let success = "SUCCESS".green().bold();
    let skipped = "SKIPPED".yellow().bold();
    let existed = "EXISTED".blue().bold();

    let print_status = |status, source: &str, target: &str| {
        println!("{: <7} :: {} {} {}", &status, &source, &arrow, &target);
    };

    link_states.iter().for_each(|link_state| {
        let source = &link_state.task.source.display().to_string();
        let target = &link_state.task.target.display().to_string();

        match &link_state.result {
            LinkResult::Existed => {
                print_status(&existed, &source, &target);
            }
            LinkResult::Skipped => {
                print_status(&skipped, &source, &target);
            }
            LinkResult::Success => {
                print_status(&success, &source, &target);
            }
            LinkResult::IoError(e) => {
                print_status(&error, &source, &target);
                println!("\t{}", e);
            }
        }
    })
}
