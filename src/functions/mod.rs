pub fn is_git_repo() -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .unwrap();

    if !output.status.success() {
        return false;
    }

    true
}
