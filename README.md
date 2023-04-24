# MinCD

Wait until a remote git repository has been changed, and then exit.
This is meant to be used in scripts, to replace a CD system.
I am using it to deploy [renn.es](https://renn.es) to my server using docker.

## Installation
```bash
cargo install mincd
```

## Usage
```bash
mincd <url> <branch> [interval]
```

Example:
```bash
mincd https://github.com/tarneaux/.f master 5
```
