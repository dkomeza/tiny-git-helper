use crate::functions::commit::File;
use inquire::{list_option::ListOption, validator::Validation};

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
            commit_specific_files(vec![]);
        }
        "Commit all files" => {
            commit_all_files(vec![]);
        }
        _ => {
            println!("Invalid option");
        }
    }
}

struct CommitOptions {
    no_push: bool,
    skip_fancy: bool,
    force_fancy: bool,
}
impl CommitOptions {
    fn new(args: Vec<String>) -> CommitOptions {
        let mut no_push = false;
        let mut skip_fancy = false;
        let mut force_fancy = false;

        args.iter().for_each(|arg| {
            if arg == "--no-push" {
                no_push = true;
            }
            if arg == "--skip-fancy" {
                skip_fancy = true;
            }
            if arg == "--force-fancy" {
                force_fancy = true;
            }
        });

        CommitOptions {
            no_push,
            skip_fancy,
            force_fancy,
        }
    }
}

pub fn commit_all_files(args: Vec<String>) {
    use crate::functions::commit::{commit_all_files, is_valid_commit};

    is_valid_commit();

    let message = ask_commit_message();

    commit_all_files(message);
}
pub fn commit_specific_files(args: Vec<String>) {
    use crate::functions::commit::{commit_specific_files, is_valid_commit};

    is_valid_commit();

    let files = ask_files_to_commit();

    let message = ask_commit_message();

    commit_specific_files(files, message);
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

            let msg = Text::new("Commit message")
                .with_help_message("Enter a commit message")
                .prompt();

            match msg {
                Ok(msg) => {
                    message += &format!(" {}", msg);
                }
                Err(_) => {
                    crate::out::print_error("Error getting commit message");
                    std::process::exit(1);
                }
            }

            let description = Text::new("Commit description")
                .with_help_message("Enter a commit description")
                .prompt();

            match description {
                Ok(description) => {
                    message += &format!("\n\n{}", description);
                }
                Err(_) => {
                    crate::out::print_error("Error getting commit description");
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
fn ask_files_to_commit() -> Vec<crate::functions::commit::File> {
    use crate::functions::commit::get_files_to_commit;
    use inquire::MultiSelect;

    let mut files = Vec::new();

    let changed_files = get_files_to_commit();

    let choice = MultiSelect::new("Select files to commit", changed_files)
        .with_validator(validate_file_selection)
        .prompt();

    match choice {
        Ok(choice) => {
            choice.iter().for_each(|file| {
                files.push(file.clone());
            });
            if files.len() == 0 {
                crate::out::print_error("You must select at least one file");
                ask_files_to_commit();
            }
        }
        Err(_) => {
            crate::out::print_error("Error getting files to commit");
            std::process::exit(1);
        }
    }

    return files;
}
fn validate_file_selection(
    files: &[ListOption<&File>],
) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
    if files.len() == 0 {
        return Ok(Validation::Invalid(
            "You must select at least one file".into(),
        ));
    }

    return Ok(Validation::Valid);
}
