#[derive(Clone)]
pub struct File {
    pub name: String,
    pub status: String,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.name)
    }
}

pub fn is_valid_commit() {
    use std::process;

    if !super::is_git_repo() {
        crate::out::print_error("Not a git repository");
        process::exit(0);
    }

    if !are_files_to_commit() {
        crate::out::print_error("No files to commit");
        process::exit(0);
    }
}

fn are_files_to_commit() -> bool {
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
pub fn get_files_to_commit() -> Vec<File> {
    use std::process::Command;

    let mut files = Vec::new();

    let output = Command::new("git")
        .arg("status")
        .arg("-s")
        .output()
        .unwrap();

    let out = String::from_utf8(output.stdout).unwrap();

    for line in out.lines() {
        let mut file = File {
            name: "".into(),
            status: "".into(),
        };

        let mut chars = line.chars();

        file.status.push(chars.next().unwrap());
        file.status.push(chars.next().unwrap());

        file.name = chars.as_str().trim().into();

        files.push(file);
    }

    return files;
}

fn commit_files(message: String, files: Vec<File>, no_push: bool) {
    use spinners::{Spinner, Spinners};
    use std::process::Command;

    let mut files_to_add = Vec::new();

    files.iter().for_each(|file| {
        files_to_add.push(file.name.clone());
    });

    let mut spinner = Spinner::new(Spinners::Dots, "Committing...".into());

    let output = Command::new("git")
        .arg("add")
        .args(files_to_add)
        .output()
        .unwrap();

    if !output.status.success() {
        crate::out::print_error("Failed to add files");

        let out = String::from_utf8(output.stderr).unwrap();
        crate::out::print_error(&out);
        std::process::exit(1);
    }

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .unwrap();

    if !output.status.success() {
        crate::out::print_error("Failed to commit files");

        let out = String::from_utf8(output.stderr).unwrap();
        crate::out::print_error(&out);
        std::process::exit(1);
    }

    if no_push {
        spinner.stop();
        return;
    }

    let output = Command::new("git").arg("push").output().unwrap();

    if !output.status.success() {
        crate::out::print_error("Failed to push files");

        let out = String::from_utf8(output.stderr).unwrap();
        crate::out::print_error(&out);
        std::process::exit(1);
    }

    spinner.stop();
}
pub fn commit_all_files(message: String, no_push: bool) {
    let files = get_files_to_commit();

    commit_files(message, files.clone(), no_push);

    println!("");
    if files.len() == 1 {
        crate::out::print_success("Successfully commited 1 file");
    } else {
        crate::out::print_success(format!("Successfully commited {} files", files.len()).as_str());
    }

    std::process::exit(0);
}
pub fn commit_specific_files(files: Vec<File>, message: String, no_push: bool) {
    commit_files(message, files.clone(), no_push);

    println!("");
    if files.len() == 1 {
        crate::out::print_success("Successfully commited 1 file");
    } else {
        crate::out::print_success(format!("Successfully commited {} files", files.len()).as_str());
    }

    std::process::exit(0);
}
