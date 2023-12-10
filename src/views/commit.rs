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
            commit_all_files();
        }
        _ => {
            println!("Invalid option");
        }
    }
}

pub fn commit_all_files() {
    use crate::functions::commit::{commit_all_files, is_valid_commit};

    is_valid_commit();

    let message = ask_commit_message();

    commit_all_files(message);
}

fn ask_commit_message() -> String {
    use inquire::{Select, Text};

    let config = crate::config::load_config();

    let mut message = String::new();

    match config.fancy {
        true => {
            let labels = crate::config::utils::get_labels();

            let icon = Select::new("Select label", labels).prompt();

            match icon {
                Ok(icon) => {
                    message += &icon.emoji;
                }
                Err(_) => {
                    crate::out::print_error("Error getting commit icon");
                    std::process::exit(1);
                }
            }
        }
        false => {
            let msg = Text::new("Commit message")
                .with_help_message("Enter a commit message")
                .prompt();

            match msg {
                Ok(msg) => {
                    message = msg;
                }
                Err(_) => {
                    crate::out::print_error("Error getting commit message");
                    std::process::exit(1);
                }
            }
        }
    }

    return message;
}
