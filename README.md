# MinCD

Wait until a remote git repository has been changed.
This is meant to be used in scripts, to replace a CD system.

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
