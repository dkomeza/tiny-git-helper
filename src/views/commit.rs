pub fn commit_menu() {
    use crate::{clear_screen, functions};
    use inquire::Select;
    use std::process;

    clear_screen();

    if !functions::is_git_repo() {
        crate::out::print_error("Not a git repository");
        process::exit(0);
    }

    if !functions::commit::are_files_to_commit() {
        crate::out::print_error("No files to commit");
        process::exit(0);
    }

    let choice;

    let menu = Select::new(
        "What do you want to commit?",
        vec!["Commit specific files", "Commit all files"],
    )
    .prompt();

    match menu {
        Ok(option) => {
            choice = option;
        }
        Err(_) => {
            process::exit(0);
        }
    }

    match choice {
        "Commit specific files" => {
            println!("commit");
        }
        "Commit all files" => {
            println!("commit all");
        }
        _ => {
            println!("Invalid option");
        }
    }
}
