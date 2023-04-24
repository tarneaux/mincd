# MinCD

Wait until a remote git repository has been changed, and then exit.
This is meant to be used in scripts, to replace a CD system.
I am using it to deploy [renn.es](https://renn.es) to my server using docker.

## Installation
```bash
cargo install mincd
```

## Usage
```
Usage: mincd <url> <branch> [-i <interval>] [-p <local/path>]
  url: URL of the git repository
  branch: branch to watch (usually main or master)
  interval: interval in seconds to check for changes (default: 60)
  local/path: path to the local git repository
If the local path is specified, mincd will get the first commit hash from the local repository instead of the remote one.
This is useful if you have a local mirror of a remote repository and want to keep it up to date.
```

Example:
```bash
mincd https://github.com/tarneaux/.f master -i 60
```

