// mincd: Wait until a remote git repository has been changed.

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args = get_args();
    match args {
        Some((url, branch, interval)) => watch(url, branch, interval),
        None => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn watch(url: String, branch: String, interval: Duration) {
    let last_commit = get_last_commit_hash(&url, &branch).unwrap_or_else(|| {
        println!("Could not get last commit hash, exiting");
        std::process::exit(1);
    });
    loop {
        let current_commit = match get_last_commit_hash(&url, &branch) {
            Some(commit) => commit,
            None => {
                println!("Could not get last commit hash, skipping");
                continue;
            }
        };
        if current_commit != last_commit {
            std::process::exit(0);
        }
        sleep(interval);
    }
}

fn get_last_commit_hash(url: &String, branch: &String) -> Option<String> {
    let output = Command::new("git")
        .arg("ls-remote")
        .arg(url)
        .arg(branch)
        .output()
        .expect("git ls-remote failed");
    let output = String::from_utf8(output.stdout).expect("utf8");
    if output.is_empty() {
        println!("There was an error while running git ls-remote.");
        println!("This might happen if the internet connection is down or the URL is invalid.");
        return None;
    } else {
        let mut lines = output.lines();
        let line = lines.next()?;
        let hash = line.split('\t').next()?;
        return Some(hash.to_string());
    }
}

fn print_usage() {
    println!("Usage: mincd <url> <branch> [interval]");
    println!("  url: URL of the git repository");
    println!("  branch: branch to watch (usually main or master)");
    println!("  interval: interval in seconds (default: 60)");
}

fn get_args() -> Option<(String, String, Duration)> {
    let mut args = std::env::args();
    args.next(); // skip program name
    let url = args.next()?;
    let branch = args.next()?;
    let interval = args.next();
    let interval = get_duration_from_interval_arg(interval);
    Some((url, branch, interval))
}

fn get_duration_from_interval_arg(interval: Option<String>) -> Duration {
    match interval {
        Some(interval) => {
            let interval = interval.parse::<u64>().expect("Could not parse interval");
            Duration::from_secs(interval)
        }
        None => Duration::from_secs(60),
    }
}
