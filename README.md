# Tiny git helper

[![Codacy Badge](https://app.codacy.com/project/badge/Grade/d588595e2823486190d05dccabe88b49)](https://app.codacy.com/gh/dkomeza/tiny-git-helper/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade) <br>
A small tool designed to make process of using git and github easier.

## Features

- Commit
  - [x] Commit all files
  - [x] Commit specific files
  - [ ] Revert a commit
  - [ ] Amend a commit
  - [ ] Better handling of commit errors
- Clone
  - [x] Clone a private repository
  - [ ] Clone a public repository (by searching)
  - [ ] Better handling of clone errors
  - [ ] Clone a repository with a specific branch
  - [ ] Clone a repository with a specific tag
- Login
  - [x] Login to GitHub
- Branch
  - [ ] Show the current branch
  - [ ] Create a new branch
  - [ ] Switch to a different branch
  - [ ] Delete a branch
  - [ ] Rename a branch
  - [ ] List all branches
  - [ ] List all branches with their last commit
  - [ ] List all remote branches
- Diff
  - [ ] Show diff of all files
  - [ ] Show diff of specific files
  - [ ] Show diff of a specific file
- Merge
  - [ ] Merge a branch
  - [ ] Merge a branch with a specific branch
  - [ ] Handle merge conflicts
- Pull
  - [ ] Pull from a branch
  - [ ] Handle pull conflicts
- Push
  - [ ] Push to a branch
  - [ ] Handle push errors
- Tag
  - [ ] Create a new tag
  - [ ] Delete a tag
  - [ ] List all tags
- Remote
  - [ ] Add a remote
  - [ ] Remove a remote
  - [ ] List all remotes
  - [ ] Change a remote URL
- Stash
  - [ ] Stash all changes
  - [ ] Stash specific changes
  - [ ] Apply a stash
  - [ ] Drop a stash
  - [ ] List all stashes
  - [ ] Show a stash
  - [ ] Show a stash diff
  - [ ] Pop a stash
  - [ ] Clear all stashes
- Status
  - [ ] Show status of all files
  - [ ] Show status of files in the current directory
  - [ ] Show currently staged files
- Init
  - [ ] Initialise a new repository
  - [ ] Initialise a new repository with a specific license
  - [ ] Initialise a new repository with a specific gitignore
  - [ ] Initialise a new repository with a specific README
  - [ ] Initialise a new repository with a specific branch
- History
  - [x] Show commit history
  - [x] Show commit diff
  - [x] Show commit details

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
