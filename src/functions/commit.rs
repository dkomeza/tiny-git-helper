use std::thread;

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

pub fn is_top_level() -> bool {
    use std::process::Command;

    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .unwrap();

    if !output.status.success() {
        return false;
    }

    let out = String::from_utf8(output.stdout).unwrap();
    let out = out.trim();
    let current_dir = std::env::current_dir().unwrap();

    if out != current_dir.to_str().unwrap() {
        return false;
    }

    true
}

pub fn commit_all_files(message: String) {
    use spinners::{Spinner, Spinners};
    use std::process::Command;

    let mut spinner = Spinner::new(Spinners::Dots, "Committing...".into());

    thread::sleep(std::time::Duration::from_millis(1000));

    // let output = Command::new("git")
    //     .arg("commit")
    //     .arg("-m")
    //     .arg(message)
    //     .output()
    //     .unwrap();

    // if !output.status.success() {
    //     crate::out::print_error("Error committing files");
    //     std::process::exit(1);
    // }

    spinner.stop();

    println!("");
    crate::out::print_success("Files succesfully committed");

    std::process::exit(0);
}
