pub fn get_current_branch() -> String {
    use std::process::Command;

    let output = Command::new("git")
        .arg("branch")
        .output()
        .expect("Failed to execute git branch");

    let out = String::from_utf8(output.stdout).unwrap();

    let mut branch = String::new();

    for line in out.lines() {
        if line.starts_with('*') {
            branch = line.split(" ").collect();
        }
    }

    branch.replace("*", "")
}
