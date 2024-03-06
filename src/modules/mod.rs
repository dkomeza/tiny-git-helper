pub mod commit;
pub mod clone;
pub mod help;

pub async fn menu() {
    use crate::clear_screen;
    use inquire::Select;
    use std::process;

    clear_screen();

    let choice;

    let menu = Select::new(
        "What do you want to do?",
        vec!["commit", "clone", "init", "login", "settings"],
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
        "commit" => {
            commit::commit_menu(commit::CommitOptions::default());
        }
        "clone" => {
            println!("clone");
        }
        "init" => {
            println!("init");
        }
        "login" => {
            crate::config::login().await;
        }
        "settings" => {
            println!("settings");
        }
        _ => {
            println!("Invalid option");
        }
    }
}
