use std::fmt::Display;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn wait_for_change(
    url: String,
    branch: String,
    interval: Duration,
    local_path: Option<String>,
) -> Result<(), GitWatcherError> {
    let last_commit_hash = match local_path {
        Some(path) => match get_last_local_commit_hash(&path, &branch) {
            Some(commit) => commit,
            None => {
                return Err(GitWatcherError::LocalHashCheckError);
            }
        },
        None => match get_last_remote_commit_hash(&url, &branch) {
            Some(commit) => commit,
            None => {
                return Err(GitWatcherError::RemoteHashCheckError);
            }
        },
    };
    loop {
        let current_commit_hash = match get_last_remote_commit_hash(&url, &branch) {
            Some(commit) => commit,
            None => {
                return Err(GitWatcherError::LoopError);
            }
        };
        if current_commit_hash != last_commit_hash {
            return Ok(());
        }
        sleep(interval);
    }
}

#[derive(Debug)]
pub enum GitWatcherError {
    LocalHashCheckError,
    RemoteHashCheckError,
    LoopError,
}

impl Display for GitWatcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitWatcherError::LocalHashCheckError => write!(f, "Local hash check error"),
            GitWatcherError::RemoteHashCheckError => write!(f, "Remote hash check error"),
            GitWatcherError::LoopError => write!(f, "Loop error"),
        }
    }
}

pub fn get_last_remote_commit_hash(url: &String, branch: &String) -> Option<String> {
    let output = Command::new("git")
        .arg("ls-remote")
        .arg(url)
        .arg(branch)
        .output();
    match output {
        Ok(output) => get_head_hash(output),
        Err(_) => {
            println!("There was an error while running git ls-remote.");
            println!("This might happen if the internet connection is down or the URL is invalid.");
            None
        }
    }
}

pub fn get_last_local_commit_hash(path: &String, branch: &String) -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg(branch)
        .current_dir(path)
        .output();
    match output {
        Ok(output) => get_head_hash(output),
        Err(_) => {
            println!("There was an error while running git rev-parse on the local repository.");
            None
        }
    }
}

fn get_head_hash(output: std::process::Output) -> Option<String> {
    let output = String::from_utf8(output.stdout).expect("utf8");
    if output.is_empty() {
        println!("There was an error while parsing the git output.");
        None
    } else {
        let mut lines = output.lines();
        let line = lines.next()?;
        let hash = line.split('\t').next()?;
        return Some(hash.to_string());
    }
}
