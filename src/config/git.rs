use crate::{out, view};

pub fn check_git() -> bool {
    let mut command = std::process::Command::new("git");
    command.arg("--version");

    let output = command.output().unwrap();

    if output.status.success() {
        return true;
    }

    false
}

pub enum GitConfigError {
    NameNotFound,
    EmailNotFound,
}

/// Checks if the user has a git config. (user.name, user.email)
pub fn check_git_config() -> Result<(), GitConfigError> {
    let mut command = std::process::Command::new("git");
    command.args(["config", "user.name"]);

    let output = command.output().unwrap();
    let binding = String::from_utf8(output.stdout).unwrap();
    let s = binding.trim();

    if !output.status.success() || s.len() == 0 {
        return Err(GitConfigError::NameNotFound);
    }

    let mut command = std::process::Command::new("git");
    command.args(["config", "user.email"]);

    let output = command.output().unwrap();
    let binding = String::from_utf8(output.stdout).unwrap();
    let s = binding.trim();

    if !output.status.success() || s.len() == 0 {
        return Err(GitConfigError::EmailNotFound);
    }

    Ok(())
}
