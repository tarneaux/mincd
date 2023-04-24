// mincd: Wait until a remote git repository has been changed.

use std::process::exit;
use std::thread::sleep;

mod argument_parser;
use argument_parser::Args;

mod git;
use git::{get_last_local_commit_hash, get_last_remote_commit_hash};

fn main() {
    let args = Args::new();
    wait_for_change(args);
}

fn wait_for_change(args: Args) {
    let last_commit_hash = match args.local_path {
        Some(path) => get_last_local_commit_hash(&path, &args.branch),
        None => get_last_remote_commit_hash(&args.url, &args.branch),
    }
    .unwrap_or_else(|| {
        println!("Could not get last commit hash, exiting.");
        exit(1);
    });
    loop {
        let current_commit_hash = match get_last_remote_commit_hash(&args.url, &args.branch) {
            Some(commit) => commit,
            None => {
                println!("Could not get last commit hash, skipping");
                sleep(args.interval);
                continue;
            }
        };
        if current_commit_hash != last_commit_hash {
            exit(0);
        }
        sleep(args.interval);
    }
}
