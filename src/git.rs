use std::process::Command;

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
