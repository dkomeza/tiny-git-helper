pub fn are_files_to_commit() -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .unwrap();

    if output.stdout.len() == 0 {
        return false;
    }

    true
}
