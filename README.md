# Tiny Git Helper (tgh)

<p align="center">
  <a href="https://app.codacy.com/gh/dkomeza/tiny-git-helper/dashboard">
    <img src="https://app.codacy.com/project/badge/Grade/d588595e2823486190d05dccabe88b49" alt="Codacy Badge"/>
  </a>
  <a href="https://github.com/dkomeza/tiny-git-helper/actions/workflows/build_binaries.yml">
    <img src="https://github.com/dkomeza/tiny-git-helper/actions/workflows/build_binaries.yml/badge.svg" alt="Build Status"/>
  </a>
  <a href="https://github.com/dkomeza/tiny-git-helper/actions/workflows/release_binaries.yml">
    <img src="https://github.com/dkomeza/tiny-git-helper/actions/workflows/release_binaries.yml/badge.svg" alt="Release Status"/>
  </a>
</p>

<p align="center">
  <b>Streamline your daily Git & GitHub workflow.</b><br/>
  A blazing fast CLI tool written in Rust designed to make common git operations effortless.
</p>

---

## 📦 Installation

`tgh` runs on **macOS**, **Linux**, and **Windows**.

### 🍎 macOS & 🐧 Linux

Run the following command in your terminal. It automatically detects your architecture (including Apple Silicon) and sets up your shell path.

#### Install latest version

```bash
curl -fsSL https://raw.githubusercontent.com/dkomeza/tiny-git-helper/main/scripts/install.sh | bash
```

#### OR Install a specific version

```bash
curl -fsSL https://raw.githubusercontent.com/dkomeza/tiny-git-helper/main/scripts/install.sh | bash -s -- v0.1.5
```

### 🪟 Windows

Run the following command in PowerShell:

```powershell
# Install latest version
irm https://raw.githubusercontent.com/dkomeza/tiny-git-helper/main/scripts/install.ps1 | iex
```

## 🚀 Usage

Once installed, simply run `tgh --help` to see the available commands.

```
tgh - A GitHub CLI written in Rust

Usage: tgh [COMMAND]

Commands:
  commit   Commit changes to the repository [aliases: cf]
  ca       Commit all files (Stage All + Commit)
  update   Update tgh to the latest version
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### ✨ Self-Updating

You don't need to run the install script again to get new features. tgh includes a built-in updater:

```bash
tgh update
```

This will check for the latest release on GitHub, download it, and safely replace your current binary.
