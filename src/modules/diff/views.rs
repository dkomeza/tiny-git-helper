use super::functions::*;

pub fn show_diff(options: super::DiffOptions) {
    use crate::functions::is_git_repo;
    use crate::out;

    if !is_git_repo() {
        out::print_error("Not a git repository (or any of the parent directories)");
        return;
    }

    let mut files;

    if options.file.is_some() {
        let file = options.file.unwrap();

        if !is_file_in_diff(&file) {
            out::print_error("File not in diff");
            return;
        }

        files = vec![file];
    } else {
        files = get_diff_files();
    }

    if options.selector {
        files = handle_diff_selector(files);
    }

    if files.is_empty() {
        out::print_error("No changes");
        return;
    }

    show_diff_for_files(files, options.extended);
}

fn handle_diff_selector(files: Vec<String>) -> Vec<String> {
    use inquire::MultiSelect;

    let choice = MultiSelect::new("Select files for diff", files)
        .with_validator(validate_file_selection)
        .prompt();

    match choice {
        Ok(files) => files,
        Err(_) => {
            crate::out::print_error("Error getting file selection");
            std::process::exit(1);
        }
    }
}

fn show_diff_for_files(files: Vec<String>, extended: bool) {
    use std::process::Command;

    let mut command = Command::new("git");
    command.arg("diff");
    command.arg("--color");

    if extended {
    } else {
        command.arg("--minimal");
        command.arg("--compact-summary");
    }

    for file in files {
        command.arg(file);
    }

    let output = command.output().expect("Failed to execute git diff");

    let diff = String::from_utf8_lossy(&output.stdout);
    println!("{}", diff);
}
