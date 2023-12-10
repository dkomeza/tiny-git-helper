use crate::out;

pub fn check_git() -> bool {
    let mut command = std::process::Command::new("git");
    command.arg("--version");

    let output = command.output().unwrap();

    if output.status.success() {
        return true;
    }

    return false;
}
/// Checks if the user has a git config. (user.name, user.email)
pub fn check_git_config() {
    let mut command = std::process::Command::new("git");
    command.args(["config", "--global", "user.name"]);

    let output = command.output().unwrap();

    if !output.status.success() {
        out::print_error("Error: user.name was not found in git config.\n");
        let name = ask_git_name();

        let mut command = std::process::Command::new("git");
        command.args(["config", "--global", "user.name", &name]);

        let output = command.output().unwrap();

        if !output.status.success() {
            out::print_error("Error: Failed to set user.name.\n");
            std::process::exit(1);
        }
    }

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout = stdout.trim().to_string();

    if stdout.len() == 0 {
        out::print_error("Error: user.name was not found in git config.\n");
        let name = ask_git_name();

        let mut command = std::process::Command::new("git");
        command.args(["config", "--global", "user.name", &name]);

        let output = command.output().unwrap();

        if !output.status.success() {
            out::print_error("Error: Failed to set user.name.\n");
            std::process::exit(1);
        }
    }

    let mut command = std::process::Command::new("git");
    command.args(["config", "--global", "user.email"]);

    let output = command.output().unwrap();

    if !output.status.success() {
        out::print_error("Error: user.email was not found in git config.\n");
        let email = ask_git_email();

        let mut command = std::process::Command::new("git");
        command.args(["config", "--global", "user.email", &email]);

        let output = command.output().unwrap();

        if !output.status.success() {
            out::print_error("Error: Failed to set user.email.\n");
            std::process::exit(1);
        }
    }

    let mut stdout = String::from_utf8(output.stdout).unwrap();
    stdout = stdout.trim().to_string();

    if stdout.len() == 0 {
        out::print_error("Error: user.email was not found in git config.\n");
        let email = ask_git_email();

        let mut command = std::process::Command::new("git");
        command.args(["config", "--global", "user.email", &email]);

        let output = command.output().unwrap();

        if !output.status.success() {
            out::print_error("Error: Failed to set user.email.\n");
            std::process::exit(1);
        }
    }
}

fn ask_git_name() -> String {
    let name = inquire::Text::new("Enter name used for git:")
        .with_validator(inquire::required!("Name is required."))
        .prompt();

    return name.unwrap();
}
fn ask_git_email() -> String {
    let email = inquire::Text::new("Enter email used for git:")
        .with_validator(super::utils::validate_email)
        .prompt();

    return email.unwrap();
}
