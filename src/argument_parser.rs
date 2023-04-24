use std::process::exit;
use std::time::Duration;

pub struct Args {
    pub url: String,
    pub branch: String,
    pub interval: Duration,
    pub local_path: Option<String>,
}

impl Args {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            println!("Not enough arguments");
            print_usage();
            exit(1);
        }
        let url = args[1].clone();
        let branch = args[2].clone();
        let mut interval = Duration::from_secs(60);
        let mut local_path = None;
        let mut i = 3;
        while i < args.len() {
            match args[i].as_str() {
                "-i" => {
                    i += 1;
                    interval = get_duration_from_interval_arg(args[i].clone());
                }
                "-p" => {
                    i += 1;
                    local_path = Some(args[i].clone());
                }
                _ => {
                    println!("Unrecognized argument: {}", args[i]);
                    print_usage();
                    exit(1);
                }
            }
            i += 1;
        }
        Args {
            url,
            branch,
            interval,
            local_path,
        }
    }
}

fn get_duration_from_interval_arg(interval: String) -> Duration {
    match interval.parse::<u64>() {
        Ok(d) => Duration::from_secs(d),
        Err(_) => {
            println!("Invalid interval: {}", interval);
            print_usage();
            exit(1);
        }
    }
}

pub fn print_usage() {
    println!("Usage: mincd <url> <branch> [-i <interval>] [-p <local/path>]");
    println!("  url: URL of the git repository");
    println!("  branch: branch to watch (usually main or master)");
    println!("  interval: interval in seconds to check for changes (default: 60)");
    println!("  local/path: path to the local git repository");
    println!("If the local path is specified, mincd will get the first commit hash from the local repository instead of the remote one.");
    println!("This is useful if you have a local mirror of a remote repository and want to keep it up to date.");
}
