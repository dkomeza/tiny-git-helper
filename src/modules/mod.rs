pub mod commit;
pub mod help;

pub fn menu(args: crate::Args) {
    use crate::clear_screen;
    use inquire::Select;
    use std::process;

    clear_screen();

    let choice;

    let menu = Select::new(
        "What do you want to do?",
        vec!["commit", "clone", "init", "login", "settings", "version"],
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
            commit::commit_menu(args);
        }
        "clone" => {
            println!("clone");
        }
        "init" => {
            println!("init");
        }
        "login" => {
            println!("login");
        }
        "settings" => {
            println!("settings");
        }
        "version" => {
            println!("version");
        }
        _ => {
            println!("Invalid option");
        }
    }
}
