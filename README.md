# Tiny git helper

A small tool designed to make process of using git and github easier.

## Usage

```
tgh --help
tgh - A GitHub CLI written in Rust

Usage: tgh [COMMAND]

Commands:
  commit  Open the commit menu [aliases: c]
  ca      Commit all files
  cf      Commit specific files
  clone   Clone a repository
  login   Login to GitHub
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

### Linux/Mac OS (Unix)

```bash
curl -s https://raw.githubusercontent.com/dkomeza/tiny-git-helper/main/scripts/install.sh | sudo sh
```

### Windows

Currently the only way to install tgh on Windows is to download the binary from [releases](https://github.com/dkomeza/tiny-git-helper/releases) page and add it to your PATH.
For example you can create a folder `C:\Program Files\tgh` and add it to your PATH.
