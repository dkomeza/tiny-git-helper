pub fn get_diff_files() -> Vec<String> {
    use crate::out;
    use std::process::Command;

    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .output()
        .expect("Failed to execute git diff");

    let files = String::from_utf8_lossy(&output.stdout);

    if files.is_empty() {
        out::print_error("No changes");
        return vec![];
    }

    let files: Vec<&str> = files.split('\n').collect();
    let files: Vec<String> = files
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    files
}
