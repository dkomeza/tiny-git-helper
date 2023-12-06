pub fn is_git_repo() -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    if output == "" {
        return false;
    }

    true
}

pub fn list_changed_files() {}
