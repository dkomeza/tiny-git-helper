mod help;

use inquire::{list_option::ListOption, validator::Validation};

pub fn commit_menu(args: crate::Args) {
    use crate::clear_screen;
    use inquire::Select;
    use std::process;

    clear_screen();

    if args.help {
        help::commit_help();
        return;
    }

    super::functions::is_valid_commit();

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
            commit_specific_files(args);
        }
        "Commit all files" => {
            commit_all_files(args);
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

pub fn commit_all_files(args: crate::Args) {
    use super::functions::{commit_all_files, is_valid_commit};

    if args.help {
        help::commit_all_help();
        return;
    }

    is_valid_commit();

    let options = CommitOptions::new(args.args);

    let message = ask_commit_message(&options);

    commit_all_files(message, options.no_push);
}
pub fn commit_specific_files(args: crate::Args) {
    use super::functions::{commit_specific_files, is_valid_commit};

    if args.help {
        help::commit_specific_help();
        return;
    }

    is_valid_commit();

    let files = ask_files_to_commit();

    let options = CommitOptions::new(args.args);

    let message = ask_commit_message(&options);

    commit_specific_files(files, message, options.no_push);
}

fn ask_commit_message(options: &CommitOptions) -> String {
    use inquire::{Select, Text};

    let config = crate::config::load_config();

    let mut message = String::new();

    let mut fancy = config.fancy;

    if options.force_fancy {
        fancy = true;
    }

    if options.skip_fancy {
        fancy = false;
    }

    match fancy {
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
fn ask_files_to_commit() -> Vec<super::functions::File> {
    use super::functions::get_files_to_commit;
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
    files: &[ListOption<&super::functions::File>],
) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
    if files.len() == 0 {
        return Ok(Validation::Invalid(
            "You must select at least one file".into(),
        ));
    }

    return Ok(Validation::Valid);
}
