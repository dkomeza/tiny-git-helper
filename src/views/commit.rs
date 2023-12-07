pub fn commit_menu() {
    use crate::{clear_screen, functions::commit::is_valid_commit};
    use inquire::Select;
    use std::process;

    clear_screen();

    is_valid_commit();

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
